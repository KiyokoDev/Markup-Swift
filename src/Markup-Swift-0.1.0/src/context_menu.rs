use eframe::egui::{self, Color32, Response, RichText, Ui};
use egui::widgets::text_edit::TextEditState;

use crate::app::App;

pub fn handle(ui: &mut Ui, _response: &Response, app: &mut App) {
    let response = ui.interact(
        egui::Rect::EVERYTHING,
        egui::Id::new("editor_context"),
        egui::Sense::hover(),
    );

    response.context_menu(|ui| {
        let ctx = ui.ctx().clone();

        if ui
            .add(
                egui::Button::new(RichText::new("Bold").size(13.0))
                    .fill(Color32::TRANSPARENT)
                    .min_size(egui::vec2(140.0, 24.0)),
            )
            .clicked()
        {
            wrap_selection(&ctx, app, "**", "**");
            ui.close();
        }

        if ui
            .add(
                egui::Button::new(RichText::new("Italic").size(13.0))
                    .fill(Color32::TRANSPARENT)
                    .min_size(egui::vec2(140.0, 24.0)),
            )
            .clicked()
        {
            wrap_selection(&ctx, app, "*", "*");
            ui.close();
        }

        if ui
            .add(
                egui::Button::new(RichText::new("Strikethrough").size(13.0))
                    .fill(Color32::TRANSPARENT)
                    .min_size(egui::vec2(140.0, 24.0)),
            )
            .clicked()
        {
            wrap_selection(&ctx, app, "~~", "~~");
            ui.close();
        }

        if ui
            .add(
                egui::Button::new(RichText::new("Inline Code").size(13.0))
                    .fill(Color32::TRANSPARENT)
                    .min_size(egui::vec2(140.0, 24.0)),
            )
            .clicked()
        {
            wrap_selection(&ctx, app, "`", "`");
            ui.close();
        }

        if ui
            .add(
                egui::Button::new(RichText::new("Link").size(13.0))
                    .fill(Color32::TRANSPARENT)
                    .min_size(egui::vec2(140.0, 24.0)),
            )
            .clicked()
        {
            wrap_selection(&ctx, app, "[", "](url)");
            ui.close();
        }

        ui.separator();

        if ui
            .add(
                egui::Button::new(RichText::new("Heading 1").size(13.0))
                    .fill(Color32::TRANSPARENT)
                    .min_size(egui::vec2(140.0, 24.0)),
            )
            .clicked()
        {
            prepend_line(&ctx, app, "# ");
            ui.close();
        }

        if ui
            .add(
                egui::Button::new(RichText::new("Heading 2").size(13.0))
                    .fill(Color32::TRANSPARENT)
                    .min_size(egui::vec2(140.0, 24.0)),
            )
            .clicked()
        {
            prepend_line(&ctx, app, "## ");
            ui.close();
        }

        if ui
            .add(
                egui::Button::new(RichText::new("Heading 3").size(13.0))
                    .fill(Color32::TRANSPARENT)
                    .min_size(egui::vec2(140.0, 24.0)),
            )
            .clicked()
        {
            prepend_line(&ctx, app, "### ");
            ui.close();
        }

        ui.separator();

        if ui
            .add(
                egui::Button::new(RichText::new("Blockquote").size(13.0))
                    .fill(Color32::TRANSPARENT)
                    .min_size(egui::vec2(140.0, 24.0)),
            )
            .clicked()
        {
            prepend_line(&ctx, app, "> ");
            ui.close();
        }

        if ui
            .add(
                egui::Button::new(RichText::new("Bullet List").size(13.0))
                    .fill(Color32::TRANSPARENT)
                    .min_size(egui::vec2(140.0, 24.0)),
            )
            .clicked()
        {
            prepend_line(&ctx, app, "- ");
            ui.close();
        }
    });
}

fn wrap_selection(ctx: &egui::Context, app: &mut App, left: &str, right: &str) {
    let id = egui::Id::new("md_editor");
    if let Some(state) = TextEditState::load(ctx, id) {
        if let Some(range) = state.cursor.char_range() {
            if range.primary != range.secondary {
                let start = range.primary.index.min(range.secondary.index);
                let end = range.primary.index.max(range.secondary.index);
                let tab = &mut app.tabs[app.active_tab];
                let selected = tab.markdown[start..end].to_string();
                let wrapped = format!("{}{}{}", left, selected, right);
                tab.markdown.replace_range(start..end, &wrapped);
                tab.dirty = true;
            }
        }
    }
}

fn prepend_line(ctx: &egui::Context, app: &mut App, prefix: &str) {
    let id = egui::Id::new("md_editor");
    if let Some(state) = TextEditState::load(ctx, id) {
        if let Some(range) = state.cursor.char_range() {
            let cursor = range.primary.index;
            let tab = &mut app.tabs[app.active_tab];
            let line_start = tab.markdown[..cursor]
                .rfind('\n')
                .map(|i| i + 1)
                .unwrap_or(0);
            tab.markdown.insert_str(line_start, prefix);
            tab.dirty = true;
        }
    }
}
