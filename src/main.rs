mod search_files;
use crate::search_files::{loop_files, search_file};
mod parse_args;
use crate::parse_args::parse_args;
use std::env;
use std::path::PathBuf;

pub static mut MATCHED_FILES: Vec<String> = Vec::<String>::new();

fn main() {
    let (path, target, maybe_dir) = parse_args(env::args().collect());
    match maybe_dir {
        Ok(_) => {
            loop_files(&target, maybe_dir.unwrap());
        }
        Err(_) => search_file(&target, PathBuf::from(path)),
    }
    unsafe {
        println!("{:?}", MATCHED_FILES);
    };
}
