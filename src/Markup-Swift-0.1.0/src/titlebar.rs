use eframe::egui::{self, pos2, Color32, FontId, Id, PointerButton, Rect, Sense, StrokeKind, Ui, Vec2, ViewportCommand};
use eframe::Frame;

use crate::app::App;

pub fn show(ui: &mut Ui, _frame: &mut Frame, app: &mut App) {
    let height = 36.0;
    let panel_frame = egui::Frame::new()
        .fill(Color32::from_rgb(12, 12, 14))
        .inner_margin(egui::Margin::symmetric(8i8, 0i8));

    egui::Panel::top("titlebar")
        .exact_size(height)
        .frame(panel_frame)
        .show_inside(ui, |ui| {
            ui.set_min_height(height);
            let rect = ui.max_rect();

            let response =
                ui.interact(rect, Id::new("titlebar_drag"), Sense::click_and_drag());

            if response.drag_started_by(PointerButton::Primary) {
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::StartDrag);
            }
            if response.double_clicked() {
                let maxed = ui
                    .ctx()
                    .input(|i| i.viewport().maximized.unwrap_or(false));
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::Maximized(!maxed));
            }

            ui.horizontal_centered(|ui| {
                ui.add_space(8.0);

                let file_name = if app.active_dirty() {
                    format!("{} *", app.file_name())
                } else {
                    app.file_name()
                };
                ui.label(
                    egui::RichText::new(file_name)
                        .font(FontId::proportional(13.0))
                        .color(Color32::from_rgb(180, 180, 185)),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    title_button(ui, "Close", |rect, painter, hovered| {
                        let c = rect.center();
                        let s = 5.0;
                        let col = if hovered { Color32::from_rgb(220, 100, 100) } else { Color32::from_rgb(150, 150, 155) };
                        painter.line_segment([pos2(c.x - s, c.y - s), pos2(c.x + s, c.y + s)], (1.5, col));
                        painter.line_segment([pos2(c.x + s, c.y - s), pos2(c.x - s, c.y + s)], (1.5, col));
                    }, |ctx| ctx.send_viewport_cmd(ViewportCommand::Close));

                    title_button(ui, "Maximize", |rect, painter, hovered| {
                        let c = rect.center();
                        let s = 5.5;
                        let col = if hovered { Color32::from_rgb(220, 220, 225) } else { Color32::from_rgb(150, 150, 155) };
                        let r = egui::Rect::from_center_size(c, Vec2::splat(s * 2.0));
                        painter.rect_stroke(r, 1.5, (1.5, col), StrokeKind::Inside);
                    }, |ctx| {
                        let maxed = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                        ctx.send_viewport_cmd(ViewportCommand::Maximized(!maxed))
                    });

                    title_button(ui, "Minimize", |rect, painter, hovered| {
                        let c = rect.center();
                        let s = 5.5;
                        let col = if hovered { Color32::from_rgb(220, 220, 225) } else { Color32::from_rgb(150, 150, 155) };
                        painter.line_segment([pos2(c.x - s, c.y), pos2(c.x + s, c.y)], (1.5, col));
                    }, |ctx| ctx.send_viewport_cmd(ViewportCommand::Minimized(true)));

                    ui.add_space(16.0);

                    let mode_label = match app.mode {
                        crate::app::Mode::Writing => "Focus",
                        crate::app::Mode::Focus => "Write",
                    };
                    if ui
                        .selectable_label(app.mode == crate::app::Mode::Focus, mode_label)
                        .clicked()
                    {
                        app.toggle_mode();
                    }
                });
            });
        });
}

fn title_button(
    ui: &mut Ui,
    tooltip: &str,
    draw: impl FnOnce(Rect, &egui::Painter, bool),
    action: impl FnOnce(&egui::Context),
) {
    let size = Vec2::splat(28.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let ctx = ui.ctx();

    if response.clicked() {
        action(ctx);
    }

    let hovered = response.hovered();
    let bg = if hovered {
        Color32::from_rgb(60, 60, 70)
    } else {
        Color32::TRANSPARENT
    };

    ui.painter().rect_filled(rect, 4.0, bg);
    draw(rect, ui.painter(), hovered);
    response.on_hover_text(tooltip);
}
