use crate::*;

/// Reads the content of a file and converts it to the specified type.
///
/// # Arguments
///
/// - `&str` - The path to the file to read.
///
/// # Returns
///
/// - `Result<T, Box<dyn std::error::Error + Send + Sync>>` - The converted file content or an error.
pub fn read_from_file<T>(file_path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: From<Vec<u8>>,
{
    let path: &Path = Path::new(file_path);
    let mut file: std::fs::File = std::fs::File::open(path)?;
    let mut content: Vec<u8> = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut content)?;
    Ok(T::from(content))
}

/// Gets the size of a file in bytes.
///
/// # Arguments
///
/// - `&str` - The path to the file.
///
/// # Returns
///
/// - `Option<u64>` - The file size in bytes if successful, None otherwise.
pub fn get_file_size(file_path: &str) -> Option<u64> {
    std::fs::metadata(file_path)
        .map(|metadata| Some(metadata.len()))
        .unwrap_or(None)
}
