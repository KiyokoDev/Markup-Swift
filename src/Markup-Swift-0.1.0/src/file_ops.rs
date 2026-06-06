use std::path::PathBuf;

pub fn open_dialog() -> Option<(PathBuf, String)> {
    let file = rfd::FileDialog::new()
        .add_filter("Markdown", &["md", "markdown", "mdown", "mkd"])
        .add_filter("All Files", &["*"])
        .set_title("Open Markdown File")
        .pick_file()?;

    let content = std::fs::read_to_string(&file).ok()?;
    Some((file, content))
}

pub fn save(path: Option<PathBuf>, content: &str) -> Option<PathBuf> {
    match path {
        Some(p) => {
            std::fs::write(&p, content).ok()?;
            Some(p)
        }
        None => save_dialog(content),
    }
}

pub fn save_dialog(content: &str) -> Option<PathBuf> {
    let file = rfd::FileDialog::new()
        .add_filter("Markdown", &["md"])
        .set_title("Save Markdown File")
        .save_file()?;

    std::fs::write(&file, content).ok()?;
    Some(file)
}
