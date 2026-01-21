use crate::*;

/// Moves a file from the source path to the destination path asynchronously.
///
/// # Arguments
///
/// - `&str` - The source file path.
/// - `&str` - The destination file path.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if the file was moved successfully, Err with error details otherwise.
pub async fn async_move_file(src: &str, dest: &str) -> Result<(), Error> {
    tokio::fs::rename(src, dest).await?;
    Ok(())
}

/// Moves a directory and all its contents to another location asynchronously.
///
/// # Arguments
///
/// - `&str` - The source directory path.
/// - `&str` - The destination directory path.
///
/// # Returns
///
/// - `Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + 'a>>` - A pinned boxed future that resolves to the move operation result.
pub fn async_move_dir<'a>(
    src_dir: &'a str,
    dest_dir: &'a str,
) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>> {
    Box::pin(async move {
        let src_path: &Path = Path::new(src_dir);
        let dest_path: &Path = Path::new(dest_dir);
        if dest_path.exists() {
            tokio::fs::remove_dir_all(dest_path).await?;
        }
        tokio::fs::create_dir_all(dest_path).await?;
        let mut entries: ReadDir = tokio::fs::read_dir(src_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let file_name: OsString = entry.file_name();
            let src_file_path: PathBuf = entry.path();
            let mut dest_file_path: PathBuf = PathBuf::from(dest_path);
            dest_file_path.push(file_name);
            if src_file_path.is_dir() {
                async_move_dir(
                    src_file_path.to_str().unwrap(),
                    dest_file_path.to_str().unwrap(),
                )
                .await?;
            } else if src_file_path.is_file() {
                tokio::fs::rename(&src_file_path, &dest_file_path).await?;
            }
        }
        tokio::fs::remove_dir(src_path).await?;
        Ok(())
    })
}
