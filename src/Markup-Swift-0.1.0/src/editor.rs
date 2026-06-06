use eframe::egui::{
    self, Color32, Frame, Margin, ScrollArea, TextEdit,
    containers::scroll_area::ScrollBarVisibility,
};

use crate::app::App;
use crate::context_menu;
use crate::wrap;

pub fn show(ui: &mut egui::Ui, app: &mut App) {
    let editor_frame = Frame::new()
        .fill(Color32::from_rgb(20, 20, 22))
        .inner_margin(egui::Margin::symmetric(8i8, 8i8));

    let text_frame = Frame::new()
        .fill(Color32::from_rgb(20, 20, 22))
        .inner_margin(Margin::symmetric(4i8, 2i8));

    editor_frame.show(ui, |ui| {
        ScrollArea::vertical()
            .id_salt("editor_scroll")
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                wrap::handle(ui.ctx(), app.current_markdown_mut());

                ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(app.current_markdown_mut())
                        .id(egui::Id::new("md_editor"))
                        .font(egui::TextStyle::Monospace)
                        .frame(text_frame)
                        .desired_width(f32::INFINITY)
                        .desired_rows(50)
                        .lock_focus(false),
                );

                let response = ui.interact(
                    egui::Rect::EVERYTHING,
                    egui::Id::new("editor_area"),
                    egui::Sense::hover(),
                );

                context_menu::handle(ui, &response, app);
            });
    });
}
