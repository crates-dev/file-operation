use std::fs::{OpenOptions, create_dir_all};
use std::io::{Error, Write};

/// Writes content to a file.
///
/// # Arguments
///
/// - `&str` - The path to the file.
/// - `&[u8]` - The content to write.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if successful, Err with error details otherwise.
pub fn write_to_file(file_path: &str, content: &[u8]) -> Result<(), Error> {
    if let Some(parent_dir) = std::path::Path::new(file_path).parent() {
        create_dir_all(parent_dir)?;
    }
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .and_then(|mut file| file.write_all(content))
}

/// Appends content to a file.
///
/// # Arguments
///
/// - `&str` - The path to the file.
/// - `&[u8]` - The content to append.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if successful, Err with error details otherwise.
pub fn append_to_file(file_path: &str, content: &[u8]) -> Result<(), Error> {
    if let Some(parent_dir) = std::path::Path::new(file_path).parent() {
        create_dir_all(parent_dir)?;
    }
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .and_then(|mut file| file.write_all(content))
}
