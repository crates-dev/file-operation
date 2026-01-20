//! file-operation
//!
//! A Rust library providing comprehensive utilities for file operations with both sync/async support.
//! Includes operations for copy, delete, move, read and write files. Simplifies file handling
//! in Rust projects with safe and efficient methods for file manipulation and metadata querying.

pub(crate) mod copy;
pub(crate) mod delete;
pub(crate) mod file;
pub(crate) mod r#move;
pub(crate) mod read;
pub(crate) mod write;

pub use {copy::*, delete::*, file::*, r#move::*, read::*, write::*};

pub(crate) use std::fmt;
