use crate::*;

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
        std::fs::create_dir_all(parent_dir)?;
    }
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .and_then(|mut file| std::io::Write::write_all(&mut file, content))
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
        std::fs::create_dir_all(parent_dir)?;
    }
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .and_then(|mut file| std::io::Write::write_all(&mut file, content))
}
