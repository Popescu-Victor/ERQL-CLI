use rfd::FileDialog;
use std::path::PathBuf;

pub fn select_file() -> Option<PathBuf> {
    let file = FileDialog::new().pick_file();
    if let Some(path) = &file {
        println!("Selected file: {}", path.display());
    } else {
        println!("No file selected.");
    }
    file
}