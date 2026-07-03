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

pub struct FilePath {
    pub selected_file: Option<PathBuf>,
}

impl FilePath {
    pub fn new() -> Self {
        FilePath { selected_file: None }
    }

    pub fn select_file(&mut self) {
        self.selected_file = select_file();
    }

    pub fn print_path(&self) {
        match &self.selected_file {
            Some(path) => println!("Selected file path: {}", path.display()),
            None => println!("No file selected."),
        }
    }
}