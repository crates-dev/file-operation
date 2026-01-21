use crate::*;

/// Copies a file from the source path to the destination path.
///
/// # Arguments
///
/// - `&str` - The source file path.
/// - `&str` - The destination file path.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if the file was copied successfully, Err with error details otherwise.
pub fn copy_file(src: &str, dest: &str) -> Result<(), Error> {
    std::fs::copy(src, dest)?;
    Ok(())
}

/// Copies all files from the source directory to the destination directory.
///
/// # Arguments
///
/// - `&str` - The source directory path.
/// - `&str` - The destination directory path.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if all files were copied successfully, Err with error details otherwise.
pub fn copy_dir_files(src_dir: &str, dest_dir: &str) -> Result<(), Error> {
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
    std::fs::create_dir_all(dest_path)?;
    for entry in std::fs::read_dir(src_path)? {
        let entry: DirEntry = entry?;
        let file_name: OsString = entry.file_name();
        let src_file_path: PathBuf = entry.path();
        let mut dest_file_path: PathBuf = PathBuf::from(dest_path);
        dest_file_path.push(file_name);
        if src_file_path.is_dir() {
            copy_dir_files(
                src_file_path.to_str().unwrap(),
                dest_file_path.to_str().unwrap(),
            )?;
        } else if src_file_path.is_file() {
            std::fs::copy(&src_file_path, &dest_file_path)?;
        }
    }
    Ok(())
}
