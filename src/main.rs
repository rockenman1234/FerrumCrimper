use std::env;

// Definition of messages as constants
const UNKOWN_FLAG_MESSAGE: &str = "Unknown flag Use --help for usage information.";
const HELP_MESSAGE: &str = r#"
ferrumcrimper: A simple command-line tool for crimping files
First option must be a mode flag:
  --hello       Print 'Hello, World!'
  --goodbye     Print 'Goodbye, World!'
Useage:
    FerrumCrimper [FLAGS] [infile] [outfile]
OPTIONS:
  --license    Print the license information
  --help       Display this help message
"#;
const GNU_LICENSE_MESSAGE: &str = r#"
Ferrum Crimper Version 0.0.1, 
Copyright (C) 2024 Alan A. Aguilar, Kenneth A. Jenkins,
and contributors. License GPLv2+: GNU GPL version 2 or later.
Ferrum Crimper comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it
under certain conditions; type `FerrumCrimper --help' for help."#;

fn main() {
    // Collect the command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check if any flags are passed
    if args.len() > 1 {
        match args[1].as_str() {
            "--hello" => {
                println!("Hello, World!");
            }
            "--goodbye" => {
                println!("Goodbye, World!");
            }
            "--license" => {
                println!("{}", GNU_LICENSE_MESSAGE);
            }
            "--help" => {
                println!("{}", HELP_MESSAGE);
            }
            _ => {
                println!("{}", UNKOWN_FLAG_MESSAGE);
            }
        }
    } else {
        println!("ferrumcrimper: Must specify a flag, use '--help for more info' \n {}", GNU_LICENSE_MESSAGE);
    }
}