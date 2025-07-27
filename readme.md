<center>

## file-operation

[![](https://img.shields.io/crates/v/file-operation.svg)](https://crates.io/crates/file-operation)
[![](https://img.shields.io/crates/d/file-operation.svg)](https://img.shields.io/crates/d/file-operation.svg)
[![](https://docs.rs/file-operation/badge.svg)](https://docs.rs/file-operation)
[![](https://github.com/eastspire/file-operation/workflows/Rust/badge.svg)](https://github.com/eastspire/file-operation/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/file-operation.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/file-operation/)

[Api Docs](https://docs.rs/file-operation/latest/file_operation/)

> A Rust library providing comprehensive utilities for file operations with both sync/async support. Includes operations for copy, delete, move, read and write files. Simplifies file handling in Rust projects with safe and efficient methods for file manipulation and metadata querying.

## Installation

To use this crate, you can run cmd:

```shell
cargo add file-operation
```

## Use

### Write to File

**Code**

```rust
let _ = write_to_file(FILE_PATH, "test".as_bytes());
```

**Description**
Writes the given data (`"test".as_bytes()`) to the file specified by `FILE_PATH`.

- `FILE_PATH` - Path to the target file.
- Returns - A `Result` indicating success or failure.

### Read from File

**Code**

```rust
let res: Vec<u8> = read_from_file(FILE_PATH).unwrap_or_default();
```

**Description**
Reads the contents of the file specified by `FILE_PATH`.

- `FILE_PATH` - Path to the target file.
- Returns - A `Vec<u8>` containing the file content or an empty vector on failure.

### Get File Size

**Code**

```rust
let size: Option<u64> = get_file_size(FILE_PATH);
```

**Description**
Retrieves the size of the file specified by `FILE_PATH`.

- `FILE_PATH` - Path to the target file.
- Returns - An `Option<u64>` containing the file size in bytes or `None` if the file does not exist.

### Copy Directory Files

**Code**

```rust
let res: Result<(), std::io::Error> = copy_dir_files(FILE_DIR, NEW_FILE_DIR);
```

**Description**
Copies all files from `FILE_DIR` to `NEW_FILE_DIR`.

- `FILE_DIR` - Source directory path.
- `NEW_FILE_DIR` - Destination directory path.
- Returns - A `Result` indicating success or failure.

### Delete File

**Code**

```rust
let res: Result<(), std::io::Error> = delete_file(FILE_PATH);
```

**Description**
Deletes the file specified by `FILE_PATH`.

- `FILE_PATH` - Path to the target file.
- Returns - A `Result` indicating success or failure.

### Move Directory

**Code**

```rust
let res: Result<(), std::io::Error> = move_dir(FILE_DIR, NEW_TEST_DIR);
```

**Description**
Moves the directory specified by `FILE_DIR` to `NEW_TEST_DIR`.

- `FILE_DIR` - Source directory path.
- `NEW_TEST_DIR` - Destination directory path.
- Returns - A `Result` indicating success or failure.

### Delete Directory

**Code**

```rust
let res: Result<(), std::io::Error> = delete_dir(NEW_TEST_DIR);
```

**Description**
Deletes the directory specified by `NEW_TEST_DIR`.

- `NEW_TEST_DIR` - Path to the target directory.
- Returns - A `Result` indicating success or failure.

### Asynchronous Write to File

**Code**

```rust
let _ = async_write_to_file(FILE_PATH, "test".as_bytes()).await;
```

**Description**
Writes the given data (`"test".as_bytes()`) to the file specified by `FILE_PATH` asynchronously.

- `FILE_PATH` - Path to the target file.
- Returns - A `Result` indicating success or failure.

### Asynchronous Read from File

**Code**

```rust
let res: Vec<u8> = async_read_from_file(FILE_PATH).await.unwrap_or_default();
```

**Description**
Reads the contents of the file specified by `FILE_PATH` asynchronously.

- `FILE_PATH` - Path to the target file.
- Returns - A `Vec<u8>` containing the file content or an empty vector on failure.

### Asynchronous Get File Size

**Code**

```rust
let size: Option<u64> = async_get_file_size(FILE_PATH).await;
```

**Description**
Retrieves the size of the file specified by `FILE_PATH` asynchronously.

- `FILE_PATH` - Path to the target file.
- Returns - An `Option<u64>` containing the file size in bytes or `None` if the file does not exist.

### Asynchronous Copy Directory Files

**Code**

```rust
let res: Result<(), std::io::Error> = async_copy_dir_files(FILE_DIR, NEW_FILE_DIR).await;
```

**Description**
Copies all files from `FILE_DIR` to `NEW_FILE_DIR` asynchronously.

- `FILE_DIR` - Source directory path.
- `NEW_FILE_DIR` - Destination directory path.
- Returns - A `Result` indicating success or failure.

### Asynchronous Delete File

**Code**

```rust
let res: Result<(), std::io::Error> = async_delete_file(FILE_PATH).await;
```

**Description**
Deletes the file specified by `FILE_PATH` asynchronously.

- `FILE_PATH` - Path to the target file.
- Returns - A `Result` indicating success or failure.

### Asynchronous Move Directory

**Code**

```rust
let res: Result<(), std::io::Error> = async_move_dir(FILE_DIR, NEW_TEST_DIR).await;
```

**Description**
Moves the directory specified by `FILE_DIR` to `NEW_TEST_DIR` asynchronously.

- `FILE_DIR` - Source directory path.
- `NEW_TEST_DIR` - Destination directory path.
- Returns - A `Result` indicating success or failure.

### Asynchronous Delete Directory

**Code**

```rust
let res: Result<(), std::io::Error> = async_delete_dir(NEW_TEST_DIR).await;
```

**Description**
Deletes the directory specified by `NEW_TEST_DIR` asynchronously.

- `NEW_TEST_DIR` - Path to the target directory.
- Returns - A `Result` indicating success or failure.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
