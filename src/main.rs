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


const HELP_TEXT: &str = "/
Welcome to ERQL! Here's a brief overview of how this program works:

Unlike other reporting software like Excel and PowerBI, ERQL works entirely based on simple scripts. \n\n

Here's an example of a basic script you can try: \n 
file>upload>csv \n

There are three parts in this command, which are conveniently named after parts of speech. There's a subject (file), a verb (upload) and an object (csv). This command tells the program you wish to:
\n 1. Have a file perform an action.
\n 2. Have that action be an upload (of itself in this case)
\n 3. Have the type of file be .csv.

\n\n Although ERQL can work with Excel files as well, to avoid formatting errors caused by using different versions we strongly recommend saving your files in .csv before working on them.
\n\n Enter "help>" + one of the following 'verbs' to get more information:
\n\n graph - for creating graphs and charts
\n file - for storing .csv files in the program's memory
\n homework - for parsing through homework data
\n virtual_class - for parsing through vc data
\n save - for saving the graph or the text into a separate file
\n group - for matching students to their tutor and storing these relations in a database

"
