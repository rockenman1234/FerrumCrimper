// Copyright 2024 the FerrumCrimper authors. All rights reserved. GNUv2 license.

mod zip;

use crate::zip::zip_utils::zip_folder;
use std::env;
use std::path::Path;

// Definition of messages as constants
const UNKOWN_FLAG_MESSAGE: &str = "Unknown flag Use --help for usage information.";
const HELP_MESSAGE: &str = r#"
ferrumcrimper: A simple command-line tool for crimping files

USEAGE:
    FerrumCrimper [FLAGS] [infile] [outfile]
First option must be a mode flag:
  --hello       Print 'Hello, World!'
OPTIONS:
  --license    Print the license information
  --help       Display this help message
  --zip, -z    Zip a folder
"#;
const GNU_LICENSE_MESSAGE: &str = r#"
Ferrum Crimper Version 0.0.1,

Copyright (C) 2024 Alan D. Aguilar, Kenneth A. Jenkins,
and contributors. License GPLv2+: GNU GPL version 2 or later.
Ferrum Crimper comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it
under certain conditions; type `fec --help' for help."#;

fn main() {
    // Collect the command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check if any flags are passed
    if args.len() > 1 {
        match args[1].as_str() {
            "--hello" => {
                println!("Hello, World!");
            }
            "--license" => {
                println!("{}", GNU_LICENSE_MESSAGE.trim_start());
            }
            "--help" | "-h" => {
                println!("{}", HELP_MESSAGE.trim_start());
            }
            "--zip" | "-z" => {
                if args.len() < 3 {
                    println!("Error: You must specify an input directory to zip.");
                    return;
                }
                let input_dir = &args[2];
                let path = Path::new(input_dir);

                match zip_folder(path) {
                    Ok(zipped_file) => {
                        println!("Successfully zipped the folder.\n Output file: {:?}", zipped_file);
                    }
                    Err(e) => {
                        println!("Failed to zip the folder:\n {}", e);
                    }
                }
            }
            _ => {
                println!("{}", UNKOWN_FLAG_MESSAGE.trim_start());
            }
        }
    } else {
        println!("ferrumcrimper: Must specify a flag, use '--help for more info' \n {}", GNU_LICENSE_MESSAGE);
    }
}