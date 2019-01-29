use std::process::exit;

pub fn no_file_error(file: &str) {
    println!("[Error] File not found: {}", file);
    exit(-1);
}