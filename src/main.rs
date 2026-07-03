use std::io::{self, Write};
use std::path::PathBuf;


mod scripts;

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let parts: Vec<&str> = command.split_whitespace().collect();


        match parts.as_slice() {
            ["file", "select"] => {
                scripts::select_file();
            }

            ["exit"] => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Unknown command: {}", command);
            }
}}}


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