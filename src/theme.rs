use egui::{Color32, Visuals};

pub fn apply(visuals: &mut Visuals) {
    *visuals = Visuals::dark();
    visuals.window_fill = Color32::from_rgb(18, 18, 20);
    visuals.panel_fill = Color32::from_rgb(22, 22, 25);
    visuals.faint_bg_color = Color32::from_rgb(28, 28, 32);
    visuals.extreme_bg_color = Color32::from_rgb(14, 14, 16);
    visuals.code_bg_color = Color32::from_rgb(30, 30, 35);
    visuals.warn_fg_color = Color32::from_rgb(240, 180, 60);
    visuals.error_fg_color = Color32::from_rgb(230, 70, 70);
    visuals.hyperlink_color = Color32::from_rgb(90, 170, 250);
    visuals.selection.bg_fill = Color32::from_rgb(60, 100, 180);
    visuals.selection.stroke.color = Color32::from_rgb(80, 130, 220);

    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(26, 26, 30);
    visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(50, 50, 55);
    visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(160, 160, 165);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(40, 40, 48);
    visuals.widgets.active.bg_fill = Color32::from_rgb(50, 50, 60);
}




