use std::io::{self, Write};
use std::path::PathBuf;


mod scripts;

fn main() {

    let mut file_path = scripts::FilePath::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let parts: Vec<&str> = command.split_whitespace().collect();


        match parts.as_slice() {
            ["file", "select"] => file_path.select_file(),

            ["file", "path"] => file_path.print_path(),

            ["exit"] => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Unknown command: {}", command);
            }
}}}

