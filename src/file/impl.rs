use crate::*;

/// Provides conversion implementation from byte vector to FileDataString.
impl From<Vec<u8>> for FileDataString {
    /// Converts a byte vector to FileDataString.
    ///
    /// # Arguments
    ///
    /// - `Vec<u8>` - The byte vector to convert.
    ///
    /// # Returns
    ///
    /// - `FileDataString` - The converted string wrapper.
    fn from(bytes: Vec<u8>) -> Self {
        FileDataString(String::from_utf8(bytes).unwrap_or_else(|_| String::new()))
    }
}

/// Provides Display trait implementation for FileDataString.
impl fmt::Display for FileDataString {
    /// Formats the FileDataString for display.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
