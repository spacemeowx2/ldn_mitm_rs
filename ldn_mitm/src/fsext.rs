use alloc::string::String;
use nx::fs;
use nx::result::*;
use nx::results;

pub const BASE_DIR: &str = "sdmc:/ldn_mitm";

pub fn ensure_directories() {
    let _ = fs::create_directory(String::from(BASE_DIR));
}
