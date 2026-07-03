use rfd::FileDialog;
use std::path::PathBuf;


struct FilePath {
    selected_file: Option<PathBuf>,
}

impl FilePath {
    fn new() -> Self {
        FilePath { selected_file: None }
    }

    fn select_file(&mut self) {
        self.selected_file = scripts::select_file();
    }

    fn print_path(&self) {
        match &self.selected_file {
            Some(path) => println!("Selected file path: {}", path.display()),
            None => println!("No file selected."),
        }
    }
}