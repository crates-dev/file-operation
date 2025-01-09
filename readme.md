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
fn test_write_to_file() {
    let _ = write_to_file(_FILE_PATH, "test".as_bytes());
}

fn test_read_from_file() {
    let res: Vec<u8> = read_from_file(_FILE_PATH).unwrap();
    println!("{:?}", String::from_utf8_lossy(&res));
}

fn test_get_file_size() {
    let size: Option<u64> = get_file_size(_FILE_PATH);
    println!("{:?}", size);
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
