// Copyright 2024 the FerrumCrimper authors. All rights reserved. GNUv2 license.

pub const VERSION_MESSAGE: &str = "FerrumCrimper(fecr) Version 0.0.6";

pub const UNKOWN_FLAG_MESSAGE: &str = "Unknown flag Use --help for usage information.";

pub const GNU_LICENSE_MESSAGE: &str = r#"
Copyright (C) 2024 - Present: Alan D. Aguilar, Kenneth A. Jenkins,
and contributors. Licensed under the GNU GPLv2: GNU General 
Public License version 2.
Ferrum Crimper comes with ABSOLUTELY NO WARRANTY.

A copy of the GNU General Public License Version 2 should 
have been provided with Ferrum Crimper. If not, you can
find it at: <https://www.gnu.org/licenses/old-licenses/gpl-2.0.html>.

This is free software, and you are welcome to redistribute it
under certain conditions, as described above. Type `fecr --help` for assistance."#;

pub const HELP_MESSAGE: &str = r#"
ferrumcrimper (fecr): A simple command-line tool for crimping files

USEAGE:
    FerrumCrimper [MODE] [INPUT PATH] [OPERATOR]

MODES:
    --license           Prints the license information.

    --help,     -h      Displays this helpful message.

    --version,  -v      Displays the version of FerrumCrimper.

    --zip,      -z      Zip a folder, you may use any of
                        the operators listed below
                        in conjunction with this flag.

    --unzip,    -uz     Unzip a folder, only the 
                        name and output operator is supported.

    --tar,      -t      Tar a folder, only the name
                        operator and output operator
                        is supported.

    --untar,    -ut     Untar a folder, only the name
                        operator and output operator
                        is supported.                                   

OPERATORS:
    --name,     -n      Specify the name of the output 
                        file or folder. This is optional,
                        and defaults to the inputed
                        folder or file name.
    
    --output,   -o      Specify the output directory for
                        the extracted files. This is optional,
                        and defaults to the current directory.

    --level,    -l      Specify the compression level to
                        use when zipping a folder. This 
                        flag is optional.
                        - Deflated levels range from 0 to 9. Default is 6.
                        - Bzip2 levels range from 0 to 9. Default is 6.
                        - Zstd levels range from -7 to 22, with zero 
                          being mapped to the level. Default is 3.

    --compression,      Specify the compression method to use.
                -c      This flag is optional, and only supports zip.
                        Supported encryption types are:
                        - bzip2 (bzip)
                        - deflate (default)
                        - zstd (z)
EXAMPLES:
    fecr --unzip /path/to/zip -n "output_folder"
    fecr --zip /path/to/folder -n output.zip -c bzip2 -l 9
    fecr --tar /path/to/folder -n output.tar -o /path/to/output
    fecr --license
"#;