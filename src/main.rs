// Copyright 2024 the FerrumCrimper authors. All rights reserved. GNUv2 license.
//  _
// | |
// | |===( )   //////
// |_|   |||  | o o|
//        ||| ( c  )                  ____
//         ||| \= /                  ||   \_
//          ||||||                   ||     |
//          ||||||                ...||__/|-"
//          ||||||             __|________|__
//            |||             |______________|
//            |||             || ||      || ||
//            |||             || ||      || ||
// -----------|||-------------||-||------||-||-------
//            |__>            || ||      || ||
//          FerrumCrimper - All nuts and bolts included.
//          See above for our compression implementation.
//          GNU GPLv2+ license. No warranties.
//          Use `fecr --help` for more information.

// Import the submodules
mod zip;
mod tar;

// Import the necessary modules from std and crates
use crate::zip::zip_utils::zip_folder;
use crate::zip::zip_utils::unzip_folder;
use std::env;
use std::path::Path;

// Definition of messages as constants
const UNKOWN_FLAG_MESSAGE: &str = "Unknown flag Use --help for usage information.";
const GNU_LICENSE_MESSAGE: &str = r#"
Ferrum Crimper Version 0.0.2,

Copyright (C) 2024 - Present: Alan D. Aguilar, Kenneth A. Jenkins,
and contributors. Licensed under the GNU GPLv2+: GNU General Public License version 2 or later.
Ferrum Crimper comes with ABSOLUTELY NO WARRANTY.

A copy of the GNU General Public License Version 2 should 
have been provided with Ferrum Crimper. If not, you can
find it at: <https://www.gnu.org/licenses/old-licenses/lgpl-2.0.html>.

This is free software, and you are welcome to redistribute it
under certain conditions, as described above. Type `fecr --help` for assistance."#;
const HELP_MESSAGE: &str = r#"
ferrumcrimper (fecr): A simple command-line tool for crimping files

USEAGE:
    FerrumCrimper [MODE] [INPUT PATH] [OPERATOR]

MODES:
    --license       Prints the license information.

    --help,  -h     Displays this helpful message.

    --zip,   -z     Zip a folder, you may use 
                    the operators listed below
                    in conjunction with this flag.

    --unzip, -uz    Unzip a folder, only the 
                    name operator is supported.

OPERATORS:
    --name,  -n     Specify the name of the output 
                    file or folder. This is optional,
                    and defaults to the inputed
                    folder or file name.

    --level, -l     Specify the compression level to
                    use when zipping a folder. This 
                    flag is optional.
                    - Zip levels range from 0 to 9. Default is 6.
                    - Deflated levels range from 0 to 9. Default is 6.
                    - Bzip2 levels range from 0 to 9. Default is 6.
                    - Zstd levels range from -7 - 22, with zero 
                      being mapped to default level. Default is 3.

    --compression,  Specify the compression method to use.
            -c      This flag is optional, and only supports zip.
                    Supported encryption types are:
                    - bzip2 (bzip)
                    - deflate (default)
                    - zstd (z)
EXAMPLES:
    ferrumcrimper --zip /path/to/folder -n output.zip -e bzip2 -l 9
    ferrumcrimper --unzip /path/to/zip -n "output_folder"
    fecr --license
"#;

fn main() {
    // Collect the command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check if any flags are passed
    if args.len() > 1 {
        match args[1].as_str() {
            "--hello" => {
                println!("Hello, World! And Go Jackets! 🐝");
            }
            "--license" => {
                println!("{}", GNU_LICENSE_MESSAGE.trim_start());
            }
            "--help" | "-h" => {
                println!("{}", HELP_MESSAGE.trim_start());
            }
            "--zip" | "-z" => {
                if args.len() < 3 {
                    println!("Error: You must specify a folder to zip.");
                    return;
                }
            
                let folder_dir = &args[2];
                let path = Path::new(folder_dir);
            
                // Initialize file_name as None by default
                let mut file_name: Option<String> = None;
                let mut encryption_type: Option<String> = None;
                let mut compression_level: Option<i64> = None;

            
                // Check if -n or --name is provided for a custom name
                let mut i = 3; // Start checking from index 3 for name-related flags
                while i < args.len() {
                    match args[i].as_str() {
                        "-n" | "--name" => {
                            if i + 1 < args.len() {
                                file_name = Some(args[i + 1].clone()); // Set the name from the next argument
                                i += 1; // Skip the next argument as it's the value for --name or -n
                            } else {
                                println!("Error: You must specify a name after the -n or --name flag.");
                                return;
                            }
                        }
                        _ => {}
                    }
                    match args[i].as_str() {
                        "-e" | "--encryption" => {
                            if i + 1 < args.len() {
                                encryption_type = Some(args[i + 1].clone()); // Set the name from the next argument
                                i += 1; // Skip the next argument as it's the value for --name or -n
                            } else {
                                println!("Error: You must specify a valid encryption type \n after the use of the -e or --encryption flag.");
                                return;
                            }
                        }
                        _ => {}
                    }
                    match args[i].as_str() {
                        "-l" | "--level" => {
                            if i + 1 < args.len() {
                                compression_level = match args[i + 1].parse::<i64>() {
                                    Ok(level) => Some(level),
                                    Err(_) => {
                                        println!("Error: Invalid compression level specified.");
                                        return;
                                    }
                                }; // Set the name from the next argument
                                i += 1; // Skip the next argument as it's the value for --name or -n
                            } else {
                                println!("Error: You must specify a valid encryption type \n after the use of the -e or --encryption flag.");
                                return;
                            }
                        }
                        _ => {}
                    }

                    i += 1;
                }
            
                // Call the zip_folder function with the folder path and file_name
                // Due to the way rust handles ownership, we explicitly need to convert the Option<String> to Option<&str>
                let file_name = file_name.as_deref();
                let encryption_type = encryption_type.as_deref();

                // Call the zip_folder function with the folder path and file_name
                match zip_folder(path, file_name, encryption_type, compression_level) {
                    Ok(zip_path) => {
                        println!("Folder zipped to: {:?}", zip_path);
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
            }
            "--unzip" | "-uz" => {
                if args.len() < 3 {
                    println!("Error: You must specify a zip file to unzip.");
                    return;
                }
            
                let zip_file = &args[2];
                let path = Path::new(zip_file);
            
                // Initialize file_name as None by default
                let mut file_name: Option<String> = None;

                // Check if -n or --name is provided
                let mut i = 3; // Start checking from index 3 for name-related flags
                while i < args.len() {
                    match args[i].as_str() {
                        "--name" | "-n" => {
                            if i + 1 < args.len() {
                                file_name = Some(args[i + 1].clone()); // Set the name from the next argument
                                i += 1; // Skip the next argument as it's the value for --name or -n
                            } 
                        }
                        _ => {}
                    }
                    i += 1;
                }

                // Call the unzip_folder function with the zip file path and file_name
                let file_name = file_name.as_deref(); // Convert Option<String> to Option<&str>
                match unzip_folder(path, file_name) {
                    Ok(extracted_path) => {
                        println!("Files extracted to: {:?}", extracted_path);
                    }
                    Err(err) => {
                        println!("Error: {}", err);
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