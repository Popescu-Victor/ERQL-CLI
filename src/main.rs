use std::io::{self, Write};


mod scripts;
mod dataframes;
mod system;
mod hw_scrape_ilias;




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
            
            ["file", "head"] => dataframes::convert_to_df(file_path.selected_file.clone()).unwrap_or_else(|e| {
                eprintln!("Error reading CSV file: {}", e);
            }),
            
            ["file", "summary"] => {}

            ["graph", "scatter", x, y] => {}

            ["graph", "hist"] => {}

            ["system", "windows"] => {
                let window_names = system::get_open_window_names();
                println!("Open Windows:");
                for name in window_names {
                    println!("{}", name);
                }
            }

            ["ilias", "scrape", weblink, localhost] => {
                if localhost.is_empty() {
                    eprintln!("Error: localhost parameter set to default.");
                    let localhost = "50098";
                    continue;
                }
                hw_scrape_ilias::scrape(weblink, localhost).unwrap();
            }

            ["exit"] => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Unknown command: {}", command);
            }
}}}

