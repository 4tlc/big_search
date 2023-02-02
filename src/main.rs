mod search_files;
use search_files::{loop_files, search_file};
mod parse_args;
use parse_args::parse_args;
mod file_size;
use file_size::search_size;
use std::path::PathBuf;
use std::str;

pub static mut MATCHED_FILES: Vec<String> = Vec::<String>::new();
pub static mut TOTAL_SIZE: u64 = 0;
pub static mut SEARCHED_SIZE: u64 = 0;

fn main() {
    let (path, target): (PathBuf, String) = parse_args(std::env::args().collect());
    println!("Gathering size of search...");
    let size: u64 = search_size(&path);
    unsafe { TOTAL_SIZE = size };
    let format_size = size
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",");
    println!("Size (in bytes): {}", format_size);
    match std::fs::read_dir(&path) {
        Ok(dir) => {
            loop_files(&target, dir);
        }
        Err(_) => {
            search_file(&target, path);
        }
    }
    unsafe {
        println!(" | Number of Matched Files: {:?}", MATCHED_FILES.len());
    };
}
