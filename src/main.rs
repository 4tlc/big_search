mod search_files;
use search_files::{loop_files, search_file};
mod parse_args;
use parse_args::parse_args;
mod file_size;
use file_size::search_size;
use std::env;
use std::path::PathBuf;

pub static mut MATCHED_FILES: Vec<String> = Vec::<String>::new();
pub static mut SEARCHED_SIZE: u64 = 0;

fn main() {
    let (path, target): (PathBuf, String) = parse_args(env::args().collect());
    println!("Size: {}", search_size(&path));
    begin_status();
    match std::fs::read_dir(&path) {
        Ok(dir) => {
            loop_files(&target, dir);
        }
        Err(_) => {
            search_file(&target, path);
        }
    }
    unsafe {
        println!("{:?}", MATCHED_FILES);
    };
}

fn begin_status() {}
