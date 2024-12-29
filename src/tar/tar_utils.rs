// Copyright 2024 the FerrumCrimper authors. All rights reserved. GNUv2 license.

// import the necessary modules
use std::fs::{self, File};
use tar::{Builder, Archive};
use std::path::{Path, PathBuf};
use anyhow::{self, Result, Context};

pub fn tar_folder(
    folder_dir: &Path,
    file_name: Option<&str>,
    output_dir: Option<&str>,
) -> Result<PathBuf> {
    // Ensure the folder exists
    if !folder_dir.is_dir() {
        anyhow::bail!("The provided folder path does not exist or is not a directory.");
    }

    // Determine the output file name and path
    let tar_file_name = file_name.unwrap_or("archive.tar");
    let output_path = match output_dir {
        Some(dir) => Path::new(dir).join(tar_file_name),
        None => Path::new(tar_file_name).to_path_buf(),
    };

    // Create the tar archive
    let tar_file = File::create(&output_path)
        .with_context(|| format!("Failed to create tar file at {:?}", output_path))?;
    let mut tar_builder = Builder::new(tar_file);

    // Add the folder's contents to the archive
    for entry in fs::read_dir(folder_dir)
        .with_context(|| format!("Failed to read directory {:?}", folder_dir))?
    {
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        let path = entry.path();
        if path.is_file() {
            let mut file = File::open(&path)
                .with_context(|| format!("Failed to open file {:?}", path))?;
            tar_builder
                .append_file(path.file_name().unwrap(), &mut file)
                .with_context(|| format!("Failed to add file {:?} to tar archive", path))?;
        }
    }

    tar_builder
        .finish()
        .with_context(|| "Failed to finalize the tar archive")?;

    Ok(output_path)
}

pub fn untar_file(
    folder_dir: &Path,
    file_name: Option<&str>,
    output_dir: Option<&str>,
) -> Result<PathBuf> {
    // Ensure the input folder exists
    if !folder_dir.is_dir() {
        anyhow::bail!("The provided folder path does not exist or is not a directory.");
    }

    // Determine the tar file name and its full path
    let tar_file_name = file_name.unwrap_or("archive.tar");
    let tar_file_path = folder_dir.join(tar_file_name);

    // Ensure the tar file exists
    if !tar_file_path.is_file() {
        anyhow::bail!("The specified tar file does not exist: {:?}", tar_file_path);
    }

    // Determine the output directory
    let output_path = match output_dir {
        Some(dir) => Path::new(dir).to_path_buf(),
        None => folder_dir.join("extracted"),
    };

    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(&output_path)
        .with_context(|| format!("Failed to create output directory {:?}", output_path))?;

    // Open the tar file
    let tar_file = File::open(&tar_file_path)
        .with_context(|| format!("Failed to open tar file {:?}", tar_file_path))?;

    // Extract the tar archive
    let mut archive = Archive::new(tar_file);
    archive
        .unpack(&output_path)
        .with_context(|| format!("Failed to extract tar file to {:?}", output_path))?;

    Ok(output_path)
}