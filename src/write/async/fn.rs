use tokio::fs::{OpenOptions, create_dir_all};
use tokio::io::{AsyncWriteExt, Error};

/// Writes content to a file asynchronously.
///
/// # Arguments
///
/// - `&str` - The path to the file.
/// - `&[u8]` - The content to write.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if successful, Err with error details otherwise.
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

/// Appends content to a file asynchronously.
///
/// # Arguments
///
/// - `&str` - The path to the file.
/// - `&[u8]` - The content to append.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if successful, Err with error details otherwise.
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
