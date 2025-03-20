use std::fs::{OpenOptions, create_dir_all};
use std::io::{Error, Write};

/// Writes the provided content to a file at the specified `file_path`.
/// If the file does not exist, it will be created. If the file exists, the content will be appended to it.
///
/// # Parameters
/// - `file_path`: The path to the file where the content will be written.
/// - `content`: A byte slice (`&[u8]`) containing the content to be written to the file.
///
/// # Returns
/// - `Result<(), Error>`:
///     - `Ok(())`: If the content was successfully written to the file.
///     - `Err(Error)`: If there was an error during file creation or writing.
///
/// # Errors
/// - If the file cannot be created or opened for writing, an error will be returned. This can happen if:
///     - There is a problem with the file path (e.g., invalid or inaccessible path).
///     - There are I/O issues when writing to the file.
#[inline]
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

/// Append the provided content to a file at the specified `file_path`.
/// If the file does not exist, it will be created. If the file exists, the content will be appended to it.
///
/// # Parameters
/// - `file_path`: The path to the file where the content will be written.
/// - `content`: A byte slice (`&[u8]`) containing the content to be written to the file.
///
/// # Returns
/// - `Result<(), Error>`:
///     - `Ok(())`: If the content was successfully written to the file.
///     - `Err(Error)`: If there was an error during file creation or writing.
///
/// # Errors
/// - If the file cannot be created or opened for writing, an error will be returned. This can happen if:
///     - There is a problem with the file path (e.g., invalid or inaccessible path).
///     - There are I/O issues when writing to the file.
#[inline]
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
