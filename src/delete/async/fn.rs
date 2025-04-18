use std::path::Path;
use tokio::fs::{remove_dir_all, remove_file};
use tokio::io::Error;

/// Asynchronously deletes a file at the given path.
///
/// - `path`: The file path to delete.
///
/// - Returns: `Ok(())` if the file was deleted successfully, or an `Err` with the error details.
pub async fn async_delete_file(path: &str) -> Result<(), Error> {
    remove_file(path).await
}

/// Asynchronously deletes a directory and all its contents.
///
/// - `path`: The directory path to delete.
///
/// - Returns: `Ok(())` if the directory and its contents were deleted successfully, or an `Err` with the error details.
pub async fn async_delete_dir(path: &str) -> Result<(), Error> {
    let dir_path: &Path = Path::new(path);
    remove_dir_all(dir_path).await?;
    Ok(())
}
