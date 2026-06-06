use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily};

pub fn configure(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    macro_rules! load_font {
        ($name:expr, $path:expr) => {
            fonts.font_data.insert(
                $name.into(),
                Arc::new(FontData::from_static(include_bytes!($path))),
            );
        };
    }

    load_font!("Lexend-Regular", "../resources/fonts/Lexend-Regular.ttf");
    load_font!("Lexend-Bold", "../resources/fonts/Lexend-Bold.ttf");
    load_font!("JetBrainsMono-Regular", "../resources/fonts/JetBrainsMono-Regular.ttf");
    load_font!("JetBrainsMono-Bold", "../resources/fonts/JetBrainsMono-Bold.ttf");
    load_font!("JetBrainsMono-Italic", "../resources/fonts/JetBrainsMono-Italic.ttf");
    load_font!("JetBrainsMono-BoldItalic", "../resources/fonts/JetBrainsMono-BoldItalic.ttf");

    let prop = fonts.families.get_mut(&FontFamily::Proportional).unwrap();
    prop.clear();
    prop.push("Lexend-Regular".into());
    prop.push("Lexend-Bold".into());

    fonts
        .families
        .entry(FontFamily::Name("bold".into()))
        .or_insert_with(|| vec!["Lexend-Bold".into()]);

    let mono = fonts.families.get_mut(&FontFamily::Monospace).unwrap();
    mono.clear();
    mono.push("JetBrainsMono-Regular".into());
    mono.push("JetBrainsMono-Bold".into());
    mono.push("JetBrainsMono-Italic".into());
    mono.push("JetBrainsMono-BoldItalic".into());

    ctx.set_fonts(fonts);
}
