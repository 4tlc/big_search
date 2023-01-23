use std::fs;
use std::fs::{DirEntry, ReadDir};

fn main() {
    let mut files: Vec<DirEntry> = Vec::<DirEntry>::new();
    let paths: ReadDir = fs::read_dir("./").unwrap();
    get_files(&mut files, paths);
    println!("{}", files.len());
}

fn get_files(files: &mut Vec<DirEntry>, paths: ReadDir) -> () {
    for path in paths {
        let p = fs::read_dir(path.as_ref().unwrap().path());
        match p {
            Ok(_) => {
                get_files(files, p.unwrap());
            }
            Err(_) => {
                files.push(path.unwrap());
            }
        }
    }
}
