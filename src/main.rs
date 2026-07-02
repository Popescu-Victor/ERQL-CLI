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
        
        for part in &parts {
            println!("{}", part);
}
    }
}