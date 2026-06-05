#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod animation;
mod app;
mod context_menu;
mod editor;
mod file_ops;
mod font;
mod preview;
mod tab_switcher;
mod theme;
mod titlebar;
mod wrap;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_min_inner_size([600.0, 400.0])
            .with_inner_size([960.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Markup Swift",
        options,
        Box::new(|cc| {
            font::configure(&cc.egui_ctx);

            let pp = cc.egui_ctx.pixels_per_point();
            cc.egui_ctx.set_pixels_per_point(pp.round().max(1.0));

            let mut visuals = egui::Visuals::dark();
            theme::apply(&mut visuals);
            cc.egui_ctx.set_visuals(visuals);

            Ok(Box::new(app::App::new()))
        }),
    )
}
