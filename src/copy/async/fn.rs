use crate::*;

/// Asynchronously copies a file from the source path to the destination path.
///
/// # Arguments
///
/// - `&str` - The source file path.
/// - `&str` - The destination file path.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if the file was copied successfully, Err with error details otherwise.
pub async fn async_copy_file(src: &str, dest: &str) -> Result<(), Error> {
    tokio::fs::copy(src, dest).await?;
    Ok(())
}

/// Asynchronously copies all files from the source directory to the destination directory.
///
/// # Arguments
///
/// - `&str` - The source directory path.
/// - `&str` - The destination directory path.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if all files were copied successfully, Err with error details otherwise.
pub async fn async_copy_dir_files(src_dir: &str, dest_dir: &str) -> Result<(), Error> {
    let src_path: &Path = Path::new(src_dir);
    let dest_path: &Path = Path::new(dest_dir);
    if dest_path.exists()
        && let Some(dest_path_str) = dest_path.to_str()
    {
        if dest_path.is_file() {
            delete_file(dest_path_str)?;
        }
        if dest_path.is_dir() {
            delete_dir(dest_path_str)?;
        }
    }
    tokio::fs::create_dir_all(dest_path).await?;
    let mut tasks: Vec<JoinHandle<Result<(), Error>>> = Vec::new();
    let mut read_dir: ReadDir = tokio::fs::read_dir(src_path).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        let file_name: OsString = entry.file_name();
        let src_file_path: PathBuf = entry.path();
        let mut dest_file_path: PathBuf = PathBuf::from(dest_path);
        dest_file_path.push(file_name);
        if src_file_path.is_dir() {
            let src_file_path_str: String = src_file_path.to_str().unwrap().to_string();
            let dest_file_path_str: String = dest_file_path.to_str().unwrap().to_string();
            tasks.push(spawn(async move {
                async_copy_file(&src_file_path_str, &dest_file_path_str).await
            }));
        } else if src_file_path.is_file() {
            let src_file_path_str: String = src_file_path.to_str().unwrap().to_string();
            let dest_file_path_str: String = dest_file_path.to_str().unwrap().to_string();
            tasks.push(spawn(async move {
                async_copy_file(&src_file_path_str, &dest_file_path_str).await
            }));
        }
    }
    for task in tasks {
        task.await??;
    }
    Ok(())
}
