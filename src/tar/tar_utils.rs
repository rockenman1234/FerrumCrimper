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

    // Get the folder's name to use as the base file name
    let folder_name = folder_dir
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to determine folder name"))?;

    // Determine the base tar file name
    let base_tar_name = file_name.unwrap_or(folder_name);
    let mut tar_file_name = format!("{}.tar", base_tar_name);

    // Determine the output path
    let output_path = match output_dir {
        Some(dir) => Path::new(dir).to_path_buf(),
        None => folder_dir.parent().unwrap_or_else(|| Path::new(".")).to_path_buf(),
    };

    // Resolve conflicts with existing file names by appending a number
    let mut full_output_path = output_path.join(&tar_file_name);
    let mut counter = 1;
    while full_output_path.exists() {
        tar_file_name = format!("{}-{}.tar", base_tar_name, counter);
        full_output_path = output_path.join(&tar_file_name);
        counter += 1;
    }

    // Create the tar archive
    let tar_file = File::create(&full_output_path)
        .with_context(|| format!("Failed to create tar file at {:?}", full_output_path))?;
    let mut tar_builder = Builder::new(tar_file);

    // Add the folder's contents to the archive
    for entry in fs::read_dir(folder_dir)
        .with_context(|| format!("Failed to read directory {:?}", folder_dir))?
    {
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        let path = entry.path();

        // Recursively add files and directories
        if path.is_file() {
            let mut file = File::open(&path)
                .with_context(|| format!("Failed to open file {:?}", path))?;
            tar_builder
                .append_file(path.strip_prefix(folder_dir).unwrap(), &mut file)
                .with_context(|| format!("Failed to add file {:?} to tar archive", path))?;
        } else if path.is_dir() {
            tar_builder
                .append_dir_all(path.strip_prefix(folder_dir).unwrap(), &path)
                .with_context(|| format!("Failed to add directory {:?} to tar archive", path))?;
        }
    }

    tar_builder
        .finish()
        .with_context(|| "Failed to finalize the tar archive")?;

    Ok(full_output_path)
}

pub fn untar_file(
    tar_file_path: &Path,
    file_name: Option<&str>,
    output_dir: Option<&str>,
) -> Result<PathBuf> {
    // Ensure the input tar file exists
    if !tar_file_path.is_file() {
        anyhow::bail!("The provided tar file path does not exist or is not a file.");
    }

    // Get the tar file's name to use as the base name if no name is provided
    let tar_file_name = tar_file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to determine tar file name"))?;

    let base_name = file_name.unwrap_or_else(|| tar_file_name.trim_end_matches(".tar"));

    // Determine the base output directory
    let base_output_path = match output_dir {
        Some(dir) => Path::new(dir).to_path_buf(),
        None => tar_file_path.parent().unwrap_or_else(|| Path::new(".")).join(format!("{}_extracted", base_name)),
    };

    // Resolve conflicts by appending a number to the output directory
    let mut output_path = base_output_path.clone();
    let mut counter = 1;
    while output_path.exists() {
        output_path = base_output_path.with_file_name(format!(
            "{}-{}",
            base_output_path.file_name().unwrap().to_str().unwrap(),
            counter
        ));
        counter += 1;
    }

    // Create the output directory
    fs::create_dir_all(&output_path)
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