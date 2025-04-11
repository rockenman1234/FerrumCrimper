// Copyright 2024 the FerrumCrimper authors. All rights reserved. GNUv2 license.

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use anyhow::{self, Result};

pub fn gzip_file(
    input_path: &Path,
    output_file_name: Option<&str>,
    compression_level: Option<u32>,
    output_dir: Option<&str>,
) -> Result<PathBuf> {
    if !input_path.is_file() {
        anyhow::bail!("Provided path is not a file: {:?}", input_path);
    }

    let file_name = input_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?
        .to_string_lossy()
        .into_owned();

    let gz_name = output_file_name
        .map(|n| PathBuf::from(n))
        .unwrap_or_else(|| PathBuf::from(format!("{}.gz", file_name)));

    let gz_path = output_dir
        .map(Path::new)
        .unwrap_or_else(|| input_path.parent().unwrap_or_else(|| Path::new(".")))
        .join(gz_name);

    let input_file = File::open(input_path)?;
    let output_file = File::create(&gz_path)?;

    let compression = Compression::new(compression_level.unwrap_or(6));
    let mut encoder = GzEncoder::new(BufWriter::new(output_file), compression);
    std::io::copy(&mut BufReader::new(input_file), &mut encoder)?;
    encoder.finish()?;

    Ok(gz_path)
}

pub fn gunzip_file(
    gz_path: &Path,
    output_file_name: Option<&str>,
    output_dir: Option<&str>,
) -> Result<PathBuf> {
    if !gz_path.is_file() {
        anyhow::bail!("Provided path is not a file: {:?}", gz_path);
    }

    let default_name = gz_path
        .file_stem()
        .ok_or_else(|| anyhow::anyhow!("Invalid .gz file name"))?
        .to_string_lossy()
        .into_owned();

    let out_name = output_file_name
        .map(String::from)
        .unwrap_or(default_name);

    let out_path = output_dir
        .map(Path::new)
        .unwrap_or_else(|| gz_path.parent().unwrap_or_else(|| Path::new(".")))
        .join(out_name);

    let input_file = File::open(gz_path)?;
    let mut decoder = GzDecoder::new(BufReader::new(input_file));
    let mut output_file = File::create(&out_path)?;
    std::io::copy(&mut decoder, &mut output_file)?;

    Ok(out_path)
}