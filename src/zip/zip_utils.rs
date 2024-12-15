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
//       FerrumCrimper's Zip implementation - src/zip.rs
//       See above for our compression implementation.

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::io::Read;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

pub fn zip_folder(folder: &Path) -> anyhow::Result<PathBuf> {
    // Ensure the folder exists
    if !folder.is_dir() {
        anyhow::bail!("Provided path is not a directory: {:?}", folder);
    }

    // Determine the output ZIP file path
    let zip_path = folder.with_extension("zip");

    // Create the ZIP file
    let file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in WalkDir::new(folder).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        let name = path.strip_prefix(folder)?;

        if path.is_file() {
            // Add files to the ZIP
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if path.is_dir() {
            // Add directories to the ZIP
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }

    zip.finish()?;
    Ok(zip_path)
}

// fn main() -> anyhow::Result<()> {
//     let folder = Path::new("path/to/your/folder"); // Replace with your folder path
//     let zip_path = zip_folder(folder)?;
//     println!("ZIP file created at: {:?}", zip_path);
//     Ok(())
// }
