static _FILE_PATH: &str = "./log/test.txt";

#[test]
fn test() {
    fn test_write_to_file() {
        use crate::*;
        let _ = write_to_file(_FILE_PATH, "test".as_bytes());
    }

    fn test_read_from_file() {
        use crate::*;
        let res: Vec<u8> = read_from_file(_FILE_PATH).unwrap();
        println!("{:?}", String::from_utf8_lossy(&res));
    }

    fn test_get_file_size() {
        use crate::*;
        let size: Option<u64> = get_file_size(_FILE_PATH);
        println!("{:?}", size);
    }

    test_write_to_file();
    test_read_from_file();
    test_get_file_size();
}
