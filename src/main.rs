use std::io::{self, Write};

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
            _ => {
                println!("Unknown command: {}", command);
            }
}}}