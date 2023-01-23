#![allow(unused)]
#[path = "../src/parse_args.rs"]
mod parse_args;
use crate::parse_args::parse_args;
#[path = "../src/search_files.rs"]
mod search_files;
use crate::search_files::{loop_files, search_file};
#[path = "../src/main.rs"]
mod main;
use crate::main::MATCHED_FILES;
use std::path::PathBuf;

#[test]
fn simple_cases() {
    // first entry of args contains system info
    let (path, target, maybe_dir) = parse_args(vec![
        "_".to_string(),
        "tests".to_string(),
        "all".to_string(),
    ]);
    match maybe_dir {
        Ok(_) => {
            loop_files(&target, maybe_dir.unwrap());
        }
        Err(_) => search_file(&target, PathBuf::from(path)),
    }
    let prefix: String = "tests/example/".to_string();
    unsafe {
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "in.txt")));
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "in2.txt")));
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "inner/1in.txt")));
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "inner/inner/2in.txt")));
        assert_eq!(MATCHED_FILES.len(), 4 as usize);
    }
}
