use std::{io::Error, path::Path};

/// Deletes a file at the given path.
///
/// # Arguments
///
/// - `&str` - The file path to delete.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if the file was deleted successfully, Err with error details otherwise.
pub fn delete_file(path: &str) -> Result<(), Error> {
    std::fs::remove_file(path)
}

/// Deletes a directory and all its contents.
///
/// # Arguments
///
/// - `&str` - The directory path to delete.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if the directory was deleted successfully, Err with error details otherwise.
pub fn delete_dir(path: &str) -> Result<(), Error> {
    let dir_path: &Path = Path::new(path);
    std::fs::remove_dir_all(dir_path)?;
    Ok(())
}
