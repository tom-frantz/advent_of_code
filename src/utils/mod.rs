use std::fs;
use std::path::Path;

pub fn input_string_from_file(file: &str) -> String {
    let path = Path::new(file).with_file_name("input.txt");
    fs::read_to_string(path).unwrap()
}
