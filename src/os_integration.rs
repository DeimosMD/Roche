use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn append_byte_to_file(path: &str, byte: u8) {
    let mut file:File = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    file.write(&[byte]).expect("file writing failed");
}

pub fn create_file(path: &str) {
    File::create(path).expect("file creation failed");
}

pub fn read_file_to_bytes(path: &str) -> Vec<u8> {
    return fs::read(path).unwrap();
}