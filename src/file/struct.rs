/// A wrapper struct for file data in string format.
///
/// Provides convenient access to file contents as a string.
#[derive(Debug, Clone)]
pub struct FileDataString(
    /// The actual string content of the file.
    pub(crate) String,
);
