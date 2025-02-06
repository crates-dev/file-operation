use tokio::fs::{create_dir_all, OpenOptions};
use tokio::io::{AsyncWriteExt, Error};

/// Writes the provided content to a file at the specified `file_path` asynchronously.
///
/// - `file_path`: The path to the file where the content will be written.
/// - `content`: A byte slice (`&[u8]`) containing the content to be written to the file.
///
/// - Returns: `Ok(())` if the content was successfully written to the file, or an `Err` with the error details.
///
/// # Errors
/// - If the file cannot be created or opened for writing, an error will be returned. This can happen if:
///     - There is a problem with the file path (e.g., invalid or inaccessible path).
///     - There are I/O issues when writing to the file.
pub async fn async_write_to_file(file_path: &str, content: &[u8]) -> Result<(), Error> {
    if let Some(parent_dir) = std::path::Path::new(file_path).parent() {
        create_dir_all(parent_dir).await?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .await?;
    file.write_all(content).await?;
    Ok(())
}

/// Append the provided content to a file at the specified `file_path` asynchronously.
///
/// - `file_path`: The path to the file where the content will be written.
/// - `content`: A byte slice (`&[u8]`) containing the content to be written to the file.
///
/// - Returns: `Ok(())` if the content was successfully written to the file, or an `Err` with the error details.
///
/// # Errors
/// - If the file cannot be created or opened for writing, an error will be returned. This can happen if:
///     - There is a problem with the file path (e.g., invalid or inaccessible path).
///     - There are I/O issues when writing to the file.
pub async fn async_append_to_file(file_path: &str, content: &[u8]) -> Result<(), Error> {
    if let Some(parent_dir) = std::path::Path::new(file_path).parent() {
        create_dir_all(parent_dir).await?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .await?;
    file.write_all(content).await?;
    Ok(())
}
