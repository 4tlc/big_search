#![allow(unused)] // fix unused warnings printing on *cargo test*

#[path = "../src/parse_args.rs"]
mod parse_args;
use crate::parse_args::parse_args;
#[path = "../src/search_files.rs"]
mod search_files;
use crate::search_files::{loop_files, search_file};
#[path = "../src/main.rs"]
mod main;
use crate::main::MATCHED_FILES;
use crate::main::SEARCHED_SIZE;
use crate::main::TOTAL_SIZE;
use std::time::Instant;

fn main() {
    let timer = Instant::now();
    one_word();
    many_spaces();
    many_lines();
    embedded_string();
    zero_bytes();
    println!("\n\x1b[39mTime to test: {:?}\x1b", timer.elapsed());
}

fn embedded_string() {
    let (path, target) = parse_args(vec![
        "_".to_string(),
        "tests/example".to_string(),
        "print(\"this is a string\")\nprint('h')".to_string(),
    ]);
    match std::fs::read_dir(&path) {
        Ok(dir) => {
            loop_files(&target, dir);
        }
        Err(_) => search_file(&target, path),
    }
    let prefix: String = "tests/example/".to_string();
    unsafe {
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "inner/with_quotes.py")));
        assert_eq!(MATCHED_FILES.len(), 1 as usize);
        MATCHED_FILES.clear();
        print!("\x1b[36m|\x1b[32mSearch Quotes\x1b");
    }
}

fn many_lines() {
    // first entry of args contains system info
    let (path, target) = parse_args(vec![
        "_".to_string(),
        "tests/example".to_string(),
        "this\nis\nmany\nlines\n".to_string(),
    ]);
    match std::fs::read_dir(&path) {
        Ok(dir) => {
            loop_files(&target, dir);
        }
        Err(_) => search_file(&target, path),
    }
    let prefix: String = "tests/example/".to_string();
    unsafe {
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "multy_line.txt")));
        assert_eq!(MATCHED_FILES.len(), 1 as usize);
        MATCHED_FILES.clear();
        print!("\x1b[36m|\x1b[32mMany Lines\x1b");
    }
}

fn one_word() {
    // first entry of args contains system info
    let (path, target) = parse_args(vec![
        "_".to_string(),
        "tests/example".to_string(),
        "all".to_string(),
    ]);
    match std::fs::read_dir(&path) {
        Ok(dir) => {
            loop_files(&target, dir);
        }
        Err(_) => search_file(&target, path),
    }
    let prefix: String = "tests/example/".to_string();
    unsafe {
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "in.txt")));
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "in2.txt")));
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "inner/1in.txt")));
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "inner/inner/2in.txt")));
        assert_eq!(MATCHED_FILES.len(), 4 as usize);
        MATCHED_FILES.clear();
        print!("\x1b[36m|\x1b[32mOne Word\x1b[0m");
    }
}

fn many_spaces() {
    let prefix: String = "tests/example/".to_string();
    let (path, target) = parse_args(vec![
        "_".to_string(),
        "tests/example".to_string(),
        "this one has     5spaces".to_string(),
    ]);
    match std::fs::read_dir(&path) {
        Ok(dir) => {
            loop_files(&target, dir);
        }
        Err(_) => search_file(&target, path),
    }
    unsafe {
        assert!(MATCHED_FILES.contains(&(prefix.to_owned() + "in2.txt")));
        assert_eq!(MATCHED_FILES.len(), 1 as usize);
        MATCHED_FILES.clear();
        print!("\x1b[36m|\x1b[32mMatch With Many Spaces\x1b[0m");
    }
}

// ensure passing a file/folder of zero bytes doesn't cause errors
fn zero_bytes() {
    let (path, target) = parse_args(vec![
        "_".to_string(),
        "tests/zero_bytes".to_string(),
        "this one has     5spaces".to_string(),
    ]);
    match std::fs::read_dir(&path) {
        Ok(dir) => {
            loop_files(&target, dir);
        }
        Err(_) => search_file(&target, path),
    }
    unsafe {
        MATCHED_FILES.clear();
        print!("\x1b[36m|\x1b[32mWorked on Zero Byte Folder/File\x1b[0m");
    }
}
