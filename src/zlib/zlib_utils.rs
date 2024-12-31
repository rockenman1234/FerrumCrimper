// Copyright 2024 the FerrumCrimper authors. All rights reserved. GNUv2 license.

use std::io::prelude::*;
use std::fs::File;
use flate2::{GzBuilder};
use flate2::Compression;

pub fn gz_folder() folder_dir: &Path,
file_name: Option<&str>,
output_dir: Option<&str>,
) -> Result<PathBuf> {
// Ensure the folder exists

// return palceholder
Ok(PathBuf::new())
}