// Copyright 2024 the FerrumCrimper authors. All rights reserved. GNUv2 license.

pub const VERSION_MESSAGE: &str = "FerrumCrimper(fecr) Version 0.0.7";

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
FerrumCrimper (fecr): A simple command-line tool for crimping files.

USAGE:
    fecr [MODE] [INPUT_PATH] [OPTIONS]

MODES:
    --license           Display license information.

    --help,     -h      Show this help message.

    --version,  -v      Show the version of FerrumCrimper.

    --zip,      -z      Zip a folder. You may use any of the
                        options listed below in conjunction
                        with this flag.

    --unzip,    -uz     Unzip a folder. Only the 
                        --name and --output options are supported.

    --tar,      -t      Create a tar archive of a folder. Only
                        the --name and --output options are supported.

    --untar,    -ut     Extract a tar archive. Only the 
                        --name and --output options are supported.

    --gzip,     -gz     Compress a folder with gzip. Only the
                        --name, --level, and --output options are supported.

    --ungzip,   -ugz    Decompress a gzip archive. Only the
                        --name and --output options are supported.

OPTIONS:
    --name,     -n      Specify the name of the output file or folder.
                        This is optional and defaults to the name of
                        the input folder or file.

    --output,   -o      Specify the output directory for the result.
                        Optional; defaults to the current directory.

    --level,    -l      Specify the compression level.
                        Optional; only applicable to compression modes.
                        - Deflate: 0-9 (default: 6)
                        - Bzip2:   0-9 (default: 6)
                        - Zstd:   -7-22 (0 maps to level 0, default: 3)

    --compression, -c   Specify the compression algorithm (zip only).
                        Optional. Supported values:
                        - deflate (default)
                        - bzip2 (bzip)
                        - zstd (z)

EXAMPLES:
    fecr --unzip /path/to/archive.zip -n output_folder
    fecr --zip /path/to/folder -n output.zip -c bzip2 -l 9
    fecr --tar /path/to/folder -n output.tar -o /path/to/output
    fecr --license
"#;
