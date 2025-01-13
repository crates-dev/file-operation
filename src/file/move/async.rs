use std::ffi::OsString;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use tokio::fs::{create_dir_all, read_dir, remove_dir, remove_dir_all, rename, ReadDir};
use tokio::io::Error;

/// Moves a file from the source path to the destination path asynchronously.
///
/// - `src`: The source file path.
/// - `dest`: The destination file path.
///
/// - Returns: `Ok(())` if the file was moved successfully, or an `Err` with the error details.
pub async fn async_move_file(src: &str, dest: &str) -> Result<(), Error> {
    rename(src, dest).await?;
    Ok(())
}

/// Moves a directory and all its contents to another location asynchronously.
///
/// - `src_dir`: The source directory path.
/// - `dest_dir`: The destination directory path.
///
/// - Returns: `Ok(())` if the directory and its contents were moved successfully, or an `Err` with the error details.
/// Moves a directory and all its contents to another location asynchronously.
///
/// - `src_dir`: The source directory path.
/// - `dest_dir`: The destination directory path.
///
/// - Returns: `Ok(())` if the directory and its contents were moved successfully, or an `Err` with the error details.
pub fn async_move_dir<'a>(
    src_dir: &'a str,
    dest_dir: &'a str,
) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>> {
    Box::pin(async move {
        let src_path: &Path = Path::new(src_dir);
        let dest_path: &Path = Path::new(dest_dir);
        if dest_path.exists() {
            remove_dir_all(dest_path).await?;
        }
        create_dir_all(dest_path).await?;
        let mut entries: ReadDir = read_dir(src_path).await?;
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
                rename(&src_file_path, &dest_file_path).await?;
            }
        }
        remove_dir(src_path).await?;
        Ok(())
    })
}
