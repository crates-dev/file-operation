use crate::*;

/// Moves a file from the source path to the destination path.
///
/// # Arguments
///
/// - `&str` - The source file path.
/// - `&str` - The destination file path.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if the file was moved successfully, Err with error details otherwise.
pub fn move_file(src: &str, dest: &str) -> Result<(), std::io::Error> {
    std::fs::rename(src, dest)?;
    Ok(())
}

/// Moves a directory and all its contents to another location.
///
/// # Arguments
///
/// - `&str` - The source directory path.
/// - `&str` - The destination directory path.
///
/// # Returns
///
/// - `Result<(), std::io::Error>` - Ok if the directory was moved successfully, Err with error details otherwise.
pub fn move_dir(src_dir: &str, dest_dir: &str) -> Result<(), std::io::Error> {
    let src_path: &Path = Path::new(src_dir);
    let dest_path: &Path = Path::new(dest_dir);
    if dest_path.exists() {
        std::fs::remove_dir_all(dest_path)?;
    }
    std::fs::create_dir_all(dest_path)?;
    for entry in std::fs::read_dir(src_path)? {
        let entry: std::fs::DirEntry = entry?;
        let file_name: OsString = entry.file_name();
        let src_file_path: PathBuf = entry.path();
        let mut dest_file_path: PathBuf = PathBuf::from(dest_path);
        dest_file_path.push(file_name);
        if src_file_path.is_dir() {
            move_dir(
                src_file_path.to_str().unwrap(),
                dest_file_path.to_str().unwrap(),
            )?;
        } else if src_file_path.is_file() {
            std::fs::rename(&src_file_path, &dest_file_path)?;
        }
    }
    std::fs::remove_dir(src_path)?;
    Ok(())
}
