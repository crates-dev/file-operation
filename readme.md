## file-operation

[![](https://img.shields.io/crates/v/file-operation.svg)](https://crates.io/crates/file-operation)
[![](https://docs.rs/file-operation/badge.svg)](https://docs.rs/file-operation)
[![](https://img.shields.io/crates/l/file-operation.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/file-operation/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/file-operation/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/file-operation/)

[Api Docs](https://docs.rs/file-operation/latest/file_operation/)

> A Rust library providing a set of utilities for common file operations such as reading, writing, and querying file metadata like size. It aims to simplify file handling in Rust projects, offering safe and efficient file manipulation methods.

## Installation

To use this crate, you can run cmd:

```shell
cargo add file-operation
```

## Use

```rust
use file_operation::*;
static FILE_DIR: &str = "./log";
static NEW_FILE_DIR: &str = "./new_log";
static NEW_TEST_DIR: &str = "./test_log";
static FILE_PATH: &str = "./log/test.txt";
let _ = write_to_file(FILE_PATH, "test".as_bytes());
let res: Vec<u8> = read_from_file(FILE_PATH).unwrap_or_default();
let size: Option<u64> = get_file_size(FILE_PATH);
println!("read_from_file => {:?}", String::from_utf8_lossy(&res));
println!("get_file_size => {:?}", size);
let res: Result<(), std::io::Error> = copy_dir_files(FILE_DIR, NEW_FILE_DIR);
println!("copy_dir_files => {:?}", res);
let res: Result<(), std::io::Error> = delete_file(FILE_PATH);
println!("delete_file => {:?}", res);
let res: Result<(), std::io::Error> = move_dir(FILE_DIR, NEW_TEST_DIR);
println!("move_dir => {:?}", res);
let res: Result<(), std::io::Error> = delete_dir(NEW_TEST_DIR);
println!("delete_dir => {:?}", res);
let res: Result<(), std::io::Error> = delete_dir(NEW_FILE_DIR);
println!("delete_dir => {:?}", res);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
