use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use eframe::egui;

use crate::animation::Animator;
use crate::editor;
use crate::file_ops;
use crate::preview;
use crate::tab_switcher;
use crate::titlebar;

#[derive(PartialEq)]
pub enum Mode {
    Writing,
    Focus,
}

pub struct Tab {
    pub markdown: String,
    pub file_path: Option<PathBuf>,
    pub dirty: bool,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            markdown: String::new(),
            file_path: None,
            dirty: false,
        }
    }

    pub fn file_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string()
    }
}

pub struct PreviewCache {
    pub markdown_hash: u64,
    pub code_cache: HashMap<u64, Vec<Vec<(u8, u8, u8, String)>>>,
}

impl PreviewCache {
    pub fn new() -> Self {
        Self {
            markdown_hash: 0,
            code_cache: HashMap::new(),
        }
    }

    pub fn markdown_changed(&mut self, markdown: &str) -> bool {
        let hash = {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            markdown.hash(&mut h);
            h.finish()
        };
        if hash != self.markdown_hash {
            self.markdown_hash = hash;
            self.code_cache.clear();
            true
        } else {
            false
        }
    }
}

pub struct App {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub mode: Mode,
    pub mode_anim: Animator,
    pub show_tab_switcher: bool,
    pub tab_switcher_selection: usize,
    pub preview_cache: PreviewCache,
}

impl App {
    pub fn new() -> Self {
        Self {
            tabs: vec![Tab::new()],
            active_tab: 0,
            mode: Mode::Writing,
            mode_anim: Animator::new(0.0, 0.25),
            show_tab_switcher: false,
            tab_switcher_selection: 0,
            preview_cache: PreviewCache::new(),
        }
    }

    pub fn current_markdown(&self) -> &str {
        &self.tabs[self.active_tab].markdown
    }

    pub fn current_markdown_mut(&mut self) -> &mut String {
        &mut self.tabs[self.active_tab].markdown
    }

    pub fn file_name(&self) -> String {
        self.tabs[self.active_tab].file_name()
    }

    pub fn active_dirty(&self) -> bool {
        self.tabs[self.active_tab].dirty
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Writing => Mode::Focus,
            Mode::Focus => Mode::Writing,
        };
    }

    pub fn add_tab(&mut self) -> usize {
        self.tabs.push(Tab::new());
        self.tabs.len() - 1
    }

    pub fn remove_tab(&mut self, idx: usize) {
        if self.tabs.len() <= 1 {
            return;
        }
        self.tabs.remove(idx);
        if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len() - 1;
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.show_tab_switcher {
            ctx.input_mut(|i| {
                let events: Vec<_> = i.events.drain(..).collect();
                for event in events {
                    match &event {
                        egui::Event::Key {
                            key,
                            pressed: true,
                            modifiers,
                            ..
                        } => match key {
                            egui::Key::Tab if !modifiers.shift => {
                                self.tab_switcher_selection =
                                    (self.tab_switcher_selection + 1) % (self.tabs.len() + 1);
                            }
                            egui::Key::Tab | egui::Key::ArrowLeft if modifiers.shift => {
                                if self.tab_switcher_selection > 0 {
                                    self.tab_switcher_selection -= 1;
                                } else {
                                    self.tab_switcher_selection = self.tabs.len();
                                }
                            }
                            egui::Key::ArrowRight => {
                                let next = self.tab_switcher_selection + 1;
                                self.tab_switcher_selection = next % (self.tabs.len() + 1);
                            }
                            egui::Key::ArrowLeft => {
                                if self.tab_switcher_selection > 0 {
                                    self.tab_switcher_selection -= 1;
                                } else {
                                    self.tab_switcher_selection = self.tabs.len();
                                }
                            }
                            egui::Key::ArrowDown => {
                                let next = self.tab_switcher_selection + 4;
                                if next <= self.tabs.len() {
                                    self.tab_switcher_selection = next;
                                }
                            }
                            egui::Key::ArrowUp => {
                                if self.tab_switcher_selection >= 4 {
                                    self.tab_switcher_selection -= 4;
                                }
                            }
                            egui::Key::Enter => {
                                if self.tab_switcher_selection >= self.tabs.len() {
                                    let idx = self.add_tab();
                                    self.active_tab = idx;
                                } else {
                                    self.active_tab = self.tab_switcher_selection;
                                }
                                self.show_tab_switcher = false;
                            }
                            egui::Key::Escape => {
                                self.show_tab_switcher = false;
                            }
                            _ => {
                                i.events.push(event);
                            }
                        },
                        egui::Event::Key { .. } => {}
                        _ => {
                            i.events.push(event);
                        }
                    }
                }
            });
            return;
        }

        ctx.input_mut(|i| {
            let events: Vec<_> = i.events.drain(..).collect();
            for event in events {
                match &event {
                    egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        let ctrl = modifiers.ctrl || modifiers.command;

                        match key {
                            egui::Key::O if ctrl => {
                                if let Some((path, content)) = file_ops::open_dialog() {
                                    let mut found = false;
                                    for (i, tab) in self.tabs.iter().enumerate() {
                                        if tab.file_path.as_deref() == Some(&path) {
                                            self.active_tab = i;
                                            found = true;
                                            break;
                                        }
                                    }
                                    if !found {
                                        self.tabs.push(Tab {
                                            markdown: content,
                                            file_path: Some(path),
                                            dirty: false,
                                        });
                                        self.active_tab = self.tabs.len() - 1;
                                    }
                                }
                            }
                            egui::Key::S if ctrl => {
                                let result = if modifiers.shift {
                                    file_ops::save_dialog(&self.tabs[self.active_tab].markdown)
                                } else {
                                    file_ops::save(
                                        self.tabs[self.active_tab].file_path.clone(),
                                        &self.tabs[self.active_tab].markdown,
                                    )
                                };
                                if let Some(path) = result {
                                    self.tabs[self.active_tab].file_path = Some(path);
                                    self.tabs[self.active_tab].dirty = false;
                                }
                            }
                            egui::Key::Tab if ctrl => {
                                self.show_tab_switcher = true;
                                self.tab_switcher_selection = self.tabs.len();
                            }
                            egui::Key::W if ctrl => {
                                self.remove_tab(self.active_tab);
                            }
                            egui::Key::N if ctrl => {
                                let idx = self.add_tab();
                                self.active_tab = idx;
                            }
                            egui::Key::F11 | egui::Key::F if ctrl && modifiers.shift => {
                                self.toggle_mode();
                            }
                            egui::Key::Escape if self.mode == Mode::Focus => {
                                self.mode = Mode::Writing;
                            }
                            _ => {
                                i.events.push(event);
                            }
                        }
                    }
                    egui::Event::Key { .. } => {
                        i.events.push(event);
                    }
                    _ => {
                        i.events.push(event);
                    }
                }
            }
        });
    }
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::from_rgb(0.07, 0.07, 0.08).to_array()
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.mode_anim.update(&ctx);
        self.handle_shortcuts(&ctx, frame);

        match self.mode {
            Mode::Writing => self.ui_writing(ui, frame),
            Mode::Focus => self.ui_focus(ui),
        }

        if self.show_tab_switcher {
            tab_switcher::show(&ctx, self);
        }
    }
}

impl App {
    fn ui_writing(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        titlebar::show(ui, frame, self);

        let avail = ui.available_size();

        ui.allocate_ui_with_layout(
            avail,
            egui::Layout::left_to_right(egui::Align::TOP),
            |ui| {
                let split = ui.available_width() * 0.5;
                let height = ui.available_height();

                ui.allocate_ui(egui::vec2(split, height), |ui| {
                    editor::show(ui, self);
                });

                ui.allocate_ui(egui::vec2(ui.available_width(), height), |ui| {
                    preview::show(ui, self, false);
                });
            },
        );
    }

    fn ui_focus(&mut self, ui: &mut egui::Ui) {
        let bg = egui::Frame::new()
            .fill(egui::Color32::from_rgb(14, 14, 16))
            .inner_margin(egui::Margin::symmetric(32i8, 16i8));

        egui::CentralPanel::default()
            .frame(bg)
            .show_inside(ui, |ui| {
                preview::show(ui, self, true);
            });

        let bottom = egui::Frame::new()
            .fill(egui::Color32::from_black_alpha(100))
            .inner_margin(egui::Margin::symmetric(16i8, 8i8));

        egui::Area::new(egui::Id::new("focus_status"))
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-16.0, -16.0))
            .show(ui.ctx(), |ui| {
                bottom.show(ui, |ui| {
                    let wc = self.tabs[self.active_tab]
                        .markdown
                        .split_whitespace()
                        .count();
                    let lc = self.tabs[self.active_tab].markdown.lines().count();
                    ui.label(
                        egui::RichText::new(format!("{} words  {} lines  Esc to exit", wc, lc))
                            .size(11.0)
                            .color(egui::Color32::GRAY),
                    );
                });
            });
    }
}
