use super::r#type::FileDataString;
use std::fmt;

impl From<Vec<u8>> for FileDataString {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        FileDataString(String::from_utf8(bytes).unwrap_or_else(|_| String::new()))
    }
}

impl fmt::Display for FileDataString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
