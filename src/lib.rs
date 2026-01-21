//! file-operation
//!
//! A Rust library providing comprehensive utilities for file operations with both sync/async support.
//! Includes operations for copy, delete, move, read and write files. Simplifies file handling
//! in Rust projects with safe and efficient methods for file manipulation and metadata querying.

mod copy;
mod delete;
mod file;
mod r#move;
mod read;
mod write;

pub use {copy::*, delete::*, file::*, r#move::*, read::*, write::*};

use std::{
    ffi::OsString,
    fmt,
    fs::DirEntry,
    io::Error,
    path::{Path, PathBuf},
    pin::Pin,
};

use tokio::{fs::ReadDir, spawn, task::JoinHandle};
