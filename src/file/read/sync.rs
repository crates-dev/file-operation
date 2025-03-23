use std::fs::*;
use std::io::Read;
use std::path::Path;

/// Reads the content of a file at the specified `file_path` and converts it to the type `T`.
/// The conversion is done by using the `From<Vec<u8>>` trait, which allows the file content
/// (read as raw bytes) to be converted into a type `T`.
///
/// # Parameters
/// - `file_path`: The path to the file that will be read.
///
/// # Returns
/// - `Result<T, Box<dyn std::error::Error>>`:
///     - `Ok(T)`: The file was read successfully and converted to the specified type `T`.
///     - `Err(Box<dyn std::error::Error>)`: An error occurred while opening or reading the file.
///       This will contain any errors encountered during the file operations, including I/O or conversion errors.
///
/// # Errors
/// - This function may return an error if:
///     - The file at `file_path` does not exist.
///     - There is an I/O error while reading the file.
///     - The content of the file cannot be converted to type `T`.
///
/// # Notes
/// - The function assumes that the content of the file can be represented as a byte vector (`Vec<u8>`),
///   which is then passed to `T::from` for conversion.
pub fn read_from_file<T>(file_path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: From<Vec<u8>>,
{
    let path: &Path = Path::new(file_path);
    let mut file: File = File::open(path)?;
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(T::from(content))
}

/// Retrieves the size of a file at the specified `file_path` in bytes.
///
/// # Parameters
/// - `file_path`: The path to the file whose size is to be fetched.
///
/// # Returns
/// - `Option<u64>`:
///     - `Some(size)`: The size of the file in bytes if the file exists and its metadata can be accessed.
///     - `None`: If the file doesn't exist, or there is an error retrieving the file's metadata.
///
/// # Errors
/// - If the file at `file_path` doesn't exist, or there is an issue accessing its metadata,
///   the function will return `None` to indicate the failure.
///
/// # Notes
/// - The function uses `metadata` to retrieve the file's metadata and specifically its size (`metadata.len()`).
///   If the file metadata is not accessible or the file does not exist, the function will return `None`.
///   This is useful when you want to safely handle cases where the file might not be present or readable.
pub fn get_file_size(file_path: &str) -> Option<u64> {
    metadata(file_path)
        .and_then(|metadata| Ok(Some(metadata.len())))
        .unwrap_or(None)
}
