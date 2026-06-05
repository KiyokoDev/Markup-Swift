use std::hash::Hasher;
use std::sync::LazyLock;

use eframe::egui::{
    self, Color32, Frame, Margin, Rect, ScrollArea, Sense, Ui, Vec2,
    containers::scroll_area::ScrollBarVisibility,
    text::{LayoutJob, TextFormat},
};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::app::App;

static SYNTAX_SET: LazyLock<syntect::parsing::SyntaxSet> = LazyLock::new(|| {
    syntect::parsing::SyntaxSet::load_defaults_newlines()
});

static THEME_SET: LazyLock<syntect::highlighting::ThemeSet> = LazyLock::new(|| {
    syntect::highlighting::ThemeSet::load_defaults()
});

struct MdCtx {
    list_stack: Vec<bool>,
    quote_depth: usize,
}

#[derive(Clone, Default)]
struct ActiveFormats {
    bold: bool,
    italic: bool,
    strike: bool,
    inline_code: bool,
    link: bool,
}

pub fn show(ui: &mut Ui, app: &mut App, focused: bool) {
    let max_width = if focused { 720.0 } else { f32::INFINITY };

    let (fill, margin) = if focused {
        (Color32::from_rgb(14, 14, 16), Margin::symmetric(32i8, 32i8))
    } else {
        (Color32::from_rgb(20, 20, 22), Margin::symmetric(8i8, 8i8))
    };

    let avail = ui.available_size();
    let (_, bg_response) = ui.allocate_exact_size(avail, Sense::hover());
    let outer = bg_response.rect;
    ui.painter().rect_filled(outer, 0.0, fill);

    let margin_f = Vec2::new(
        (margin.left + margin.right) as f32,
        (margin.top + margin.bottom) as f32,
    );
    let inner = Rect::from_min_size(
        outer.min + Vec2::new(margin.left as f32, margin.top as f32),
        outer.size() - margin_f,
    );

    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(inner));
    child_ui.allocate_ui_with_layout(
        child_ui.available_size(),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
        ScrollArea::vertical()
            .id_salt("preview_scroll")
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                let mut ctx = MdCtx {
                    list_stack: Vec::new(),
                    quote_depth: 0,
                };

                if max_width.is_finite() {
                    ui.set_max_width(max_width);
                }

                let markdown = app.current_markdown().to_string();
                let markdown_changed = app.preview_cache.markdown_changed(&markdown);

                let opts = Options::ENABLE_TABLES
                    | Options::ENABLE_STRIKETHROUGH
                    | Options::ENABLE_TASKLISTS
                    | Options::ENABLE_HEADING_ATTRIBUTES;

                let parser = Parser::new_ext(&markdown, opts);
                let mut line_segs: Vec<(String, ActiveFormats)> = Vec::new();
                let mut fmts = ActiveFormats::default();
                let mut in_code = false;
                let mut code_text = String::new();
                let mut code_lang = String::new();

                for event in parser {
                    match event {
                        Event::Start(tag) => match tag {
                            Tag::Paragraph => {
                                line_segs.clear();
                                fmts = ActiveFormats::default();
                            }
                            Tag::Heading { .. } => {
                                line_segs.clear();
                                fmts = ActiveFormats::default();
                            }
                            Tag::CodeBlock(kind) => {
                                in_code = true;
                                code_text.clear();
                                code_lang = match kind {
                                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                                    CodeBlockKind::Indented => String::new(),
                                };
                            }
                            Tag::List(start) => {
                                ctx.list_stack.push(start.is_some());
                            }
                            Tag::Item => {
                                line_segs.clear();
                                fmts = ActiveFormats::default();
                            }
                            Tag::BlockQuote(_) => {
                                ctx.quote_depth += 1;
                            }
                            Tag::Emphasis => fmts.italic = true,
                            Tag::Strong => fmts.bold = true,
                            Tag::Strikethrough => fmts.strike = true,
                            Tag::Link { .. } => {
                                fmts.link = true;
                            }
                            _ => {}
                        },
                        Event::End(tag_end) => match tag_end {
                            TagEnd::Paragraph => {
                                flush_line(ui, &line_segs, &ctx, true);
                                line_segs.clear();
                                fmts = ActiveFormats::default();
                            }
                            TagEnd::Heading(level) => {
                                flush_heading(ui, &line_segs, level);
                                line_segs.clear();
                                fmts = ActiveFormats::default();
                            }
                            TagEnd::CodeBlock => {
                                in_code = false;
                                render_code_block(ui, &code_text, &code_lang, &mut app.preview_cache, markdown_changed);
                            }
                            TagEnd::List(_) => {
                                ctx.list_stack.pop();
                            }
                            TagEnd::Item => {
                                if line_segs.is_empty() {
                                    render_line(ui, &[], &ctx);
                                } else {
                                    flush_line(ui, &line_segs, &ctx, false);
                                }
                                line_segs.clear();
                                fmts = ActiveFormats::default();
                            }
                            TagEnd::BlockQuote(_) => {
                                ctx.quote_depth = ctx.quote_depth.saturating_sub(1);
                            }
                            TagEnd::Emphasis => fmts.italic = false,
                            TagEnd::Strong => fmts.bold = false,
                            TagEnd::Strikethrough => fmts.strike = false,
                            TagEnd::Link => {
                                fmts.link = false;
                            }
                            _ => {}
                        },
                        Event::Text(t) => {
                            if in_code {
                                code_text.push_str(&t);
                            } else {
                                line_segs.push((t.to_string(), fmts.clone()));
                            }
                        }
                        Event::Code(t) => {
                            line_segs.push((format!("`{}`", t), ActiveFormats { inline_code: true, ..Default::default() }));
                        }
                        Event::SoftBreak | Event::HardBreak => {
                            line_segs.push(("\n".to_string(), ActiveFormats::default()));
                        }
                        Event::Rule => {
                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);
                        }
                        Event::TaskListMarker(checked) => {
                            let marker = if checked { "\u{2611}" } else { "\u{2610}" };
                            line_segs.push((format!("{} ", marker), ActiveFormats { inline_code: true, ..Default::default() }));
                        }
                        _ => {}
                    }
                }
            });
        });
}

fn make_font_id(size: f32, fmts: &ActiveFormats) -> egui::FontId {
    if fmts.bold {
        egui::FontId::new(size, egui::FontFamily::Name("bold".into()))
    } else if fmts.inline_code {
        egui::FontId::new(size, egui::FontFamily::Monospace)
    } else {
        egui::FontId::new(size, egui::FontFamily::Proportional)
    }
}

fn text_format(size: f32, fmts: &ActiveFormats) -> TextFormat {
    let color = if fmts.link {
        Color32::from_rgb(90, 170, 250)
    } else {
        Color32::from_rgb(210, 210, 215)
    };

    TextFormat {
        font_id: make_font_id(size, fmts),
        color,
        italics: fmts.italic,
        underline: if fmts.link {
            egui::Stroke::new(1.0, Color32::from_rgb(90, 170, 250))
        } else {
            egui::Stroke::NONE
        },
        strikethrough: if fmts.strike {
            egui::Stroke::new(1.0, Color32::from_rgb(120, 120, 130))
        } else {
            egui::Stroke::NONE
        },
        ..Default::default()
    }
}

fn build_job(segs: &[(String, ActiveFormats)], size: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    for (text, fmts) in segs {
        job.append(text.as_str(), 0.0, text_format(size, fmts));
    }
    job
}

fn render_segments(ui: &mut Ui, segs: &[(String, ActiveFormats)], size: f32) {
    if segs.is_empty() {
        return;
    }
    ui.label(build_job(segs, size));
}

fn render_line(ui: &mut Ui, segs: &[(String, ActiveFormats)], ctx: &MdCtx) {
    let level = ctx.list_stack.len();
    let indent = level as f32 * 16.0;
    if indent > 0.0 {
        let mut job = LayoutJob::default();
        let space = if level > 1 { indent - 16.0 } else { 0.0 };
        if space > 0.0 {
            job.append(&" ".repeat(space as usize), 0.0, TextFormat {
                font_id: egui::FontId::new(14.0, egui::FontFamily::Proportional),
                color: Color32::from_rgb(210, 210, 215),
                ..Default::default()
            });
        }
        job.append("- ", 0.0, TextFormat {
            font_id: egui::FontId::new(14.0, egui::FontFamily::Proportional),
            color: Color32::from_rgb(210, 210, 215),
            ..Default::default()
        });
        for (text, fmts) in segs {
            job.append(text.as_str(), 0.0, text_format(14.0, fmts));
        }
        ui.add(egui::Label::new(job).wrap());
    } else if ctx.quote_depth > 0 {
        quote_frame(ui, ctx.quote_depth, |ui| {
            ui.add(egui::Label::new(build_job(segs, 14.0)).wrap());
        });
    } else {
        ui.add(egui::Label::new(build_job(segs, 14.0)).wrap());
    }
}

fn flush_line(ui: &mut Ui, segs: &[(String, ActiveFormats)], ctx: &MdCtx, spacing: bool) {
    if segs.is_empty() || (segs.len() == 1 && segs[0].0.trim().is_empty()) {
        if spacing {
            ui.add_space(4.0);
        }
        return;
    }

    if spacing {
        ui.add_space(8.0);
    }

    let mut start = 0;
    for (i, (text, _)) in segs.iter().enumerate() {
        if text == "\n" {
            if i > start {
                render_line(ui, &segs[start..i], ctx);
            }
            start = i + 1;
        }
    }
    if start < segs.len() {
        render_line(ui, &segs[start..], ctx);
    }

    if spacing {
        ui.add_space(4.0);
    }
}

fn flush_heading(ui: &mut Ui, segs: &[(String, ActiveFormats)], level: HeadingLevel) {
    if segs.is_empty() {
        return;
    }
    let size = match level {
        HeadingLevel::H1 => 24.0,
        HeadingLevel::H2 => 20.0,
        HeadingLevel::H3 => 17.0,
        HeadingLevel::H4 => 15.0,
        _ => 14.0,
    };
    ui.add_space(if level == HeadingLevel::H1 {
        12.0
    } else {
        8.0
    });
    let mut start = 0;
    for (i, (text, _)) in segs.iter().enumerate() {
        if text == "\n" {
            if i > start {
                render_segments(ui, &segs[start..i], size);
            }
            start = i + 1;
        }
    }
    if start < segs.len() {
        render_segments(ui, &segs[start..], size);
    }
    ui.add_space(if level == HeadingLevel::H1 {
        4.0
    } else {
        2.0
    });
}

fn render_code_block(ui: &mut Ui, code: &str, lang: &str, cache: &mut crate::app::PreviewCache, changed: bool) {
    if code.trim().is_empty() {
        return;
    }

    ui.add_space(8.0);

    let key = {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            std::hash::Hash::hash(code, &mut h);
            std::hash::Hash::hash(lang, &mut h);
            h.finish()
    };

    let bg = Color32::from_rgb(22, 22, 28);
    let frame = Frame::new()
        .fill(bg)
        .corner_radius(6)
        .inner_margin(Margin::symmetric(12i8, 8i8));
    frame.show(ui, |ui| {
        if changed {
            let ss = &*SYNTAX_SET;
            let ts = &*THEME_SET;
            let syntax = if lang.is_empty() {
                ss.find_syntax_plain_text()
            } else {
                ss.find_syntax_by_token(lang)
                    .unwrap_or_else(|| ss.find_syntax_plain_text())
            };
            let theme = &ts.themes["base16-ocean.dark"];
            let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);

            let mut lines = Vec::new();
            for line in syntect::util::LinesWithEndings::from(code) {
                if let Ok(ranges) = highlighter.highlight_line(line, ss) {
                    let formatted: Vec<(u8, u8, u8, String)> = ranges.iter().map(|(style, text)| {
                        let fg = style.foreground;
                        (fg.r, fg.g, fg.b, text.to_string())
                    }).collect();
                    lines.push(formatted);
                }
            }
            cache.code_cache.insert(key, lines);
        }

        if let Some(lines) = cache.code_cache.get(&key) {
            for line in lines {
                let mut job = LayoutJob::default();
                for (r, g, b, text) in line {
                    let color = Color32::from_rgb(*r, *g, *b);
                    job.append(text.as_str(), 0.0, TextFormat {
                        font_id: egui::FontId::new(13.0, egui::FontFamily::Monospace),
                        color,
                        ..Default::default()
                    });
                }
                ui.add(egui::Label::new(job).wrap());
            }
        }
    });

    ui.add_space(8.0);
}

fn quote_frame(ui: &mut Ui, depth: usize, add: impl FnOnce(&mut Ui)) {
    let _ = depth;
    let color = Color32::from_rgb(80, 160, 80);

    let frame = Frame::new()
        .fill(Color32::from_rgb(25, 28, 25))
        .inner_margin(Margin::symmetric(8i8, 4i8));
    frame.show(ui, |ui| {
        let painter = ui.painter();
        let rect = ui.max_rect();
        painter.line_segment(
            [
                egui::pos2(rect.left(), rect.top()),
                egui::pos2(rect.left(), rect.bottom()),
            ],
            (3.0, color),
        );
        add(ui);
    });
}
