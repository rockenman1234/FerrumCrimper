// Copyright 2024 the FerrumCrimper authors. All rights reserved. GNUv2 license.

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::io::Read;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

pub fn zip_folder(folder_dir: &Path, file_name: Option<&str>, compression_type: Option<&str>, compression_level: Option<i64>) -> anyhow::Result<PathBuf> {
    // Ensure the folder exists
    if !folder_dir.is_dir() {
        anyhow::bail!("Provided path is not a directory, or does not exist: {:?}", folder_dir);
    }

    // Determine the output ZIP file path
    let zip_path = match file_name {
        Some(name) => folder_dir
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(name)
            .with_extension("zip"),
        None => folder_dir.with_extension("zip"),
    };


    // Determine the compression method
    let compression_method = match compression_type {
        Some("bzip2") | Some("bzip") => CompressionMethod::Bzip2,
        Some("deflate") | Some("default") => CompressionMethod::Deflated,
        Some("zstd") | Some("z") => CompressionMethod::Zstd,
        Some(invalid) => {
            panic!("Invalid compression method: '{}'", invalid);
        },
        None => CompressionMethod::Deflated,
    };

    // Determine the compression level
    let compression_amount = match compression_level {
        Some(level) => level,
        None => {
            if compression_method == CompressionMethod::Zstd {
                3
            } else {
                6
            }
        }
    };    


    // Create the ZIP file
    let file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(compression_method)
        .compression_level(Some(compression_amount))
        .unix_permissions(0o755);

    // Buffer for reading files
    let mut buffer = Vec::new();
    for entry in WalkDir::new(folder_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        let name = path.strip_prefix(folder_dir)?;

        // Add files to the ZIP
        if path.is_file() {
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

pub fn unzip_folder(zip_file_dir: &Path, file_name: Option<&str>) -> anyhow::Result<PathBuf> {
    // Ensure the file exists and is a valid file
    if !zip_file_dir.is_file() {
        anyhow::bail!("Provided path is not a file, or does not exist: {:?}", zip_file_dir);
    }

    // Determine the base output directory name
    let base_name = match file_name {
        Some(name) => name.to_string(),
        None => zip_file_dir.file_stem().unwrap_or_default().to_string_lossy().into_owned(),
    };

    // Ensure the output directory does not conflict
    let mut output_dir = zip_file_dir.parent().unwrap_or_else(|| Path::new(".")).join(&base_name);
    let mut counter = 1;
    while output_dir.exists() {
        output_dir = zip_file_dir
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(format!("{}-{}", base_name, counter));
        counter += 1;
    }

    // Create the output directory
    std::fs::create_dir_all(&output_dir)?;

    // Open the ZIP file
    let file = File::open(zip_file_dir)?;
    let mut archive = zip::ZipArchive::new(file)?;

    // Extract each file in the archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = output_dir.join(file.mangled_name());

        if file.is_dir() {
            // Create directories
            std::fs::create_dir_all(&out_path)?;
        } else {
            // Create parent directories if needed
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Write the file
            let mut outfile = File::create(&out_path)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // Set permissions if available
        #[cfg(unix)]
        if let Some(mode) = file.unix_mode() {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&out_path, std::fs::Permissions::from_mode(mode))?;
        }
    }

    Ok(output_dir)
}