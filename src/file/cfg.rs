static _FILE_PATH: &str = "./log/test.txt";

#[test]
fn test_write() {
    use crate::*;
    let _ = write_to_file(_FILE_PATH, "test".as_bytes());
    let res: Vec<u8> = read_from_file(_FILE_PATH).unwrap_or_default();
    let size: Option<u64> = get_file_size(_FILE_PATH);
    println!("test_write => {:?}", String::from_utf8_lossy(&res));
    println!("test_write => {:?}", size);
}

#[test]
fn test_copy() {
    use crate::*;
    use std::{thread::sleep, time::Duration};
    sleep(Duration::from_secs(2));
    let res: Result<(), std::io::Error> = copy_dir_files("./log", "./cp_test");
    println!("test_copy => {:?}", res);
}

#[test]
fn test_move() {
    use crate::*;
    use std::{thread::sleep, time::Duration};
    sleep(Duration::from_secs(6));
    let res: Result<(), std::io::Error> = move_dir("./log", "./test");
    println!("test_move => {:?}", res);
}

#[test]
fn test_delete() {
    use crate::*;
    use std::{thread::sleep, time::Duration};
    sleep(Duration::from_secs(10));
    let res: Result<(), std::io::Error> = delete_dir("./test");
    println!("test_delete => {:?}", res);
    let res: Result<(), std::io::Error> = delete_dir("./cp_test");
    println!("test_delete => {:?}", res);
    let res: Result<(), std::io::Error> = delete_dir(_FILE_PATH);
    println!("test_delete => {:?}", res);
}
