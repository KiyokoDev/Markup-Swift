use eframe::egui;
use egui::widgets::text_edit::TextEditState;

pub fn handle(ctx: &egui::Context, text: &mut String) {
    let id = egui::Id::new("md_editor");

    let Some(state) = TextEditState::load(ctx, id) else {
        return;
    };
    let Some(range) = state.cursor.char_range() else {
        return;
    };
    if range.primary == range.secondary {
        return;
    }

    let start = range.primary.index.min(range.secondary.index);
    let end = range.primary.index.max(range.secondary.index);

    let wrap_char = ctx.input(|i| {
        for event in &i.events {
            if let egui::Event::Text(s) = event {
                let ch = s.as_str();
                if matches!(
                    ch,
                    "*" | "`" | "\"" | "'" | "_" | "(" | "[" | "{" | "<"
                ) {
                    return Some(ch.to_string());
                }
            }
        }
        None
    });

    let Some(s) = wrap_char else { return };
    let ch = s.as_str();

    let selected = text[start..end].to_string();
    let wrapped = match ch {
        "*" => format!("*{}*", selected),
        "`" => format!("`{}`", selected),
        "\"" => format!("\"{}\"", selected),
        "'" => format!("'{}'", selected),
        "_" => format!("_{}_", selected),
        "(" => format!("({})", selected),
        "[" => format!("[{}]", selected),
        "{" => format!("{{{}}}", selected),
        "<" => format!("<{}>", selected),
        _ => return,
    };

    text.replace_range(start..end, &wrapped);

    ctx.input_mut(|i| {
        i.events.retain(|e| !matches!(e, egui::Event::Text(t) if t == &s));
    });
}
