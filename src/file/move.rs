use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

/// Moves a file from the source path to the destination path.
///
/// - `src`: The source file path.
/// - `dest`: The destination file path.
///
/// - Returns: `Ok(())` if the file was moved successfully, or an `Err` with the error details.
pub fn move_file(src: &str, dest: &str) -> Result<(), io::Error> {
    fs::rename(src, dest)?;
    Ok(())
}

/// Moves a directory and all its contents to another location.
///
/// - `src_dir`: The source directory path.
/// - `dest_dir`: The destination directory path.
///
/// - Returns: `Ok(())` if the directory and its contents were moved successfully, or an `Err` with the error details.
pub fn move_dir(src_dir: &str, dest_dir: &str) -> Result<(), io::Error> {
    let src_path: &Path = Path::new(src_dir);
    let dest_path: &Path = Path::new(dest_dir);
    if dest_path.exists() {
        fs::remove_dir_all(dest_path)?;
    }
    fs::create_dir_all(dest_path)?;
    for entry in fs::read_dir(src_path)? {
        let entry: fs::DirEntry = entry?;
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
            fs::rename(&src_file_path, &dest_file_path)?;
        }
    }
    fs::remove_dir(src_path)?;
    Ok(())
}
