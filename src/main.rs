mod search_files;
use search_files::{loop_files, search_file};
mod parse_args;
use parse_args::parse_args;
mod errors;
mod file_size;
use file_size::search_size;
use std::path::PathBuf;
use std::str;

pub static mut MATCHED_FILES: Vec<String> = Vec::<String>::new();
pub static mut TOTAL_SIZE: u64 = 0;
pub static mut SEARCHED_SIZE: u64 = 0;

fn main() {
    let parsed = parse_args(std::env::args().collect());
    let path: PathBuf;
    let target: String;
    let calc_size: bool;
    match parsed {
        Ok(p) => {
            path = p.0;
            target = p.1;
            calc_size = p.2;
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
    // let (path, target, calc_size): (PathBuf, String, bool) = parse_args(std::env::args().collect());
    if calc_size {
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
    }
    match std::fs::read_dir(&path) {
        Ok(dir) => {
            loop_files(&target, dir);
        }
        Err(_) => {
            search_file(&target, path);
        }
    }
    unsafe {
        println!("\nNumber of Matched Files: {:?}", MATCHED_FILES.len());
        for path in MATCHED_FILES.iter() {
            println!("{}", path);
        }
    };
}
