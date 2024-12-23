use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

pub fn validate_file(file_path: &Path) -> io::Result<()> {
    if !file_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist.",
        ));
    }

    let file_metadata = fs::metadata(file_path)?;
    if file_metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The path is a directory, not a file.",
        ));
    }

    let file_size = file_metadata.len();
    if file_size > MAX_FILE_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File size exceeds the maximum allowed limit.",
        ));
    }

    Ok(())
}

pub fn save_temp_file(content: &[u8], extension: &str) -> io::Result<String> {
    let temp_dir = env::temp_dir().join(format!("{}.{}", uuid::Uuid::new_v4(), extension));
    let mut temp_file = File::create(&temp_dir)?;

    temp_file.write_all(content)?;

    Ok(temp_dir.to_string_lossy().into_owned())
}

pub fn delete_temp_file(file_path: &str) -> io::Result<()> {
    if fs::remove_file(file_path).is_err() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Temporary file not found or already removed.",
        ));
    }
    Ok(())
}