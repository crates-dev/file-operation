use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::delete_dir;
use crate::delete_file;

/// Copies a file from the source path to the destination path.
///
/// - `src`: The source file path.
/// - `dest`: The destination file path.
///
/// - Returns: `Ok(())` if the file was copied successfully, or an `Err` with the error details.
pub fn copy_file(src: &str, dest: &str) -> Result<(), io::Error> {
    fs::copy(src, dest)?;
    Ok(())
}

/// Copies all files from the source directory to the destination directory.
///
/// - `src_dir`: The source directory path.
/// - `dest_dir`: The destination directory path.
///
/// - Returns: `Ok(())` if all files were copied successfully, or an `Err` with the error details.
/// Copies a directory and all its contents to another location.
///
/// - `src_dir`: The source directory path.
/// - `dest_dir`: The destination directory path.
///
/// - Returns: `Ok(())` if the directory and its contents were copied successfully, or an `Err` with the error details.
pub fn copy_dir_files(src_dir: &str, dest_dir: &str) -> Result<(), io::Error> {
    let src_path: &Path = Path::new(src_dir);
    let dest_path: &Path = Path::new(dest_dir);
    if dest_path.exists() {
        if let Some(dest_path_str) = dest_path.to_str() {
            if dest_path.is_file() {
                delete_file(dest_path_str)?;
            }
            if dest_path.is_dir() {
                delete_dir(dest_path_str)?;
            }
        }
    }
    fs::create_dir_all(dest_path)?;
    for entry in fs::read_dir(src_path)? {
        let entry: fs::DirEntry = entry?;
        let file_name = entry.file_name();
        let src_file_path: PathBuf = entry.path();
        let mut dest_file_path: PathBuf = PathBuf::from(dest_path);
        dest_file_path.push(file_name);
        if src_file_path.is_dir() {
            copy_dir_files(
                src_file_path.to_str().unwrap(),
                dest_file_path.to_str().unwrap(),
            )?;
        } else if src_file_path.is_file() {
            fs::copy(&src_file_path, &dest_file_path)?;
        }
    }
    Ok(())
}
