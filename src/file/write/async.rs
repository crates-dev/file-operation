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
///
/// # Notes
/// - The function first ensures that the parent directory of the file exists. If it doesn't, it attempts to create the entire directory structure using `create_dir_all`.
/// - The file is opened using `OpenOptions` with the following options:
///     - `write(true)`: Open the file for writing.
///     - `append(true)`: If the file exists, append the content to the end of the file.
///     - `create(true)`: If the file doesn't exist, create it.
pub async fn async_write_to_file(file_path: &str, content: &[u8]) -> Result<(), Error> {
    if let Some(parent_dir) = std::path::Path::new(file_path).parent() {
        create_dir_all(parent_dir).await?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .await?;
    file.write_all(content).await?;
    Ok(())
}
