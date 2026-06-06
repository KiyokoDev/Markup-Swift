use eframe::egui::{self, Color32, Frame, Margin, RichText, Vec2};

use crate::app::App;

const COLS: usize = 4;
const CARD_W: f32 = 110.0;
const CARD_H: f32 = 72.0;
const GAP: f32 = 6.0;
const H_MARGIN: f32 = 14.0;
const V_MARGIN: f32 = 12.0;

pub fn show(ctx: &egui::Context, app: &mut App) {
    let area_width = COLS as f32 * CARD_W + (COLS - 1) as f32 * GAP + 2.0 * H_MARGIN;
    let viewport = ctx.viewport_rect();
    let pos = egui::pos2(
        viewport.center().x - area_width / 2.0,
        viewport.center().y - 150.0,
    );

    let mut close_idx: Option<usize> = None;
    let mut switch_to: Option<usize> = None;
    let mut new_tab = false;
    let mut modal_rect = egui::Rect::NOTHING;

    let total_items = app.tabs.len() + 1;
    let rows = total_items.div_ceil(COLS);

    let area_id = egui::Id::new("tab_switcher_grid");
    egui::Area::new(area_id).fixed_pos(pos).order(egui::Order::Foreground).show(ctx, |ui| {
        let frame = Frame::new()
            .fill(Color32::from_rgb(30, 30, 35))
            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(60, 60, 65)))
            .corner_radius(10)
            .inner_margin(Margin::symmetric(H_MARGIN as i8, V_MARGIN as i8));
        frame.show(ui, |ui| {
            ui.set_min_width(area_width);
            ui.label(
                RichText::new("Switch Tab")
                    .size(13.0)
                    .color(Color32::from_rgb(160, 160, 170)),
            );
            ui.add_space(6.0);

            let sep = egui::Separator::default().spacing(0.0).grow(4.0);
            ui.add(sep);
            ui.add_space(8.0);

            let scroll_h = rows.min(3) as f32 * (CARD_H + GAP) - GAP;
            let mut need_scroll = false;

            egui::ScrollArea::vertical()
                .max_height(scroll_h)
                .scroll_bar_visibility(egui::containers::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    for row in 0..rows {
                        ui.horizontal(|ui| {
                            for col in 0..COLS {
                                let i = row * COLS + col;
                                if i >= total_items {
                                    break;
                                }

                                if i < app.tabs.len() {
                                    let tab = &app.tabs[i];
                                    let selected = i == app.tab_switcher_selection;
                                    let is_active = i == app.active_tab;

                                    let bg = if selected {
                                        Color32::from_rgb(55, 60, 72)
                                    } else if is_active {
                                        Color32::from_rgb(45, 50, 62)
                                    } else {
                                        Color32::from_rgb(35, 36, 42)
                                    };

                                    let border = if is_active {
                                        egui::Stroke::new(1.5, Color32::from_rgb(100, 180, 255))
                                    } else if selected {
                                        egui::Stroke::new(1.0, Color32::from_rgb(80, 90, 120))
                                    } else {
                                        egui::Stroke::new(1.0, Color32::from_rgb(48, 48, 55))
                                    };

                                    let label = if tab.dirty {
                                        format!("{} *", tab.file_name())
                                    } else {
                                        tab.file_name()
                                    };

                                    let inner_frame = Frame::new()
                                        .fill(bg)
                                        .stroke(border)
                                        .corner_radius(6)
                                        .inner_margin(Margin::symmetric(8i8, 6i8));

                                    let resp = inner_frame.show(ui, |ui| {
                                        ui.set_min_size(Vec2::new(CARD_W - 16.0, CARD_H - 12.0));
                                        ui.vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    RichText::new(truncate(&label, 10))
                                                        .size(11.0)
                                                        .color(Color32::from_rgb(210, 210, 215)),
                                                );
                                                if app.tabs.len() > 1 {
                                                    let cr = ui.add(
                                                        egui::Button::new(RichText::new("x").size(9.0).color(Color32::from_rgb(140, 140, 150)))
                                                            .fill(Color32::TRANSPARENT)
                                                            .min_size(Vec2::new(14.0, 14.0)),
                                                    );
                                                    if cr.clicked() {
                                                        close_idx = Some(i);
                                                    }
                                                }
                                            });
                                            if tab.dirty {
                                                ui.label(
                                                    RichText::new("modified")
                                                        .size(9.0)
                                                        .color(Color32::from_rgb(200, 160, 80)),
                                                );
                                            }
                                        });
                                    });

                                    if resp.response.clicked() {
                                        switch_to = Some(i);
                                    }
                                    if selected && !need_scroll {
                                        ui.scroll_to_rect(resp.response.rect, Some(egui::Align::Center));
                                        need_scroll = true;
                                    }
                                } else {
                                    let is_sel = app.tab_switcher_selection >= app.tabs.len();
                                    let bg = if is_sel {
                                        Color32::from_rgb(55, 60, 72)
                                    } else {
                                        Color32::from_rgb(35, 36, 42)
                                    };
                                    let border = if is_sel {
                                        egui::Stroke::new(1.5, Color32::from_rgb(100, 200, 120))
                                    } else {
                                        egui::Stroke::new(1.0, Color32::from_rgb(48, 48, 55))
                                    };

                                    let inner_frame = Frame::new()
                                        .fill(bg)
                                        .stroke(border)
                                        .corner_radius(6)
                                        .inner_margin(Margin::symmetric(8i8, 6i8));

                                    let resp = inner_frame.show(ui, |ui| {
                                        ui.set_min_size(Vec2::new(CARD_W - 16.0, CARD_H - 12.0));
                                        ui.vertical_centered(|ui| {
                                            ui.add_space(4.0);
                                            ui.label(
                                                RichText::new("+")
                                                    .size(24.0)
                                                    .color(Color32::from_rgb(130, 220, 150)),
                                            );
                                            ui.label(
                                                RichText::new("New Tab")
                                                    .size(10.0)
                                                    .color(Color32::from_rgb(130, 220, 150)),
                                            );
                                        });
                                    });

                                    if resp.response.clicked() {
                                        new_tab = true;
                                    }
                                    if is_sel && !need_scroll {
                                        ui.scroll_to_rect(resp.response.rect, Some(egui::Align::Center));
                                        need_scroll = true;
                                    }
                                }
                            }
                        });
                    }
                });

            modal_rect = ui.min_rect();
        });
    });

    let outside_click = ctx.input(|i| {
        if i.pointer.any_click() {
            i.pointer.latest_pos().map(|p| !modal_rect.contains(p)).unwrap_or(false)
        } else {
            false
        }
    });
    if outside_click {
        app.show_tab_switcher = false;
        return;
    }

    if let Some(idx) = close_idx {
        app.remove_tab(idx);
        if app.tab_switcher_selection >= app.tabs.len() {
            app.tab_switcher_selection = app.tabs.len().saturating_sub(1);
        }
    }
    if let Some(idx) = switch_to {
        app.active_tab = idx;
        app.show_tab_switcher = false;
    }
    if new_tab {
        let idx = app.add_tab();
        app.active_tab = idx;
        app.show_tab_switcher = false;
    }
}

fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(max_chars.saturating_sub(1)).collect::<String>())
    }
}
