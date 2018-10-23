use std::fs;

pub fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("FILE_NOT_FOUND")
}