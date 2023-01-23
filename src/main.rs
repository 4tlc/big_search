use std::fs;
use std::fs::{DirEntry, ReadDir};

fn main() {
    let mut files: Vec<DirEntry> = Vec::<DirEntry>::new();
    let paths: ReadDir = fs::read_dir("./").unwrap();
    get_files_old(&mut files, paths);
    println!("{}", files.len());
    let mut files: Vec<DirEntry> = Vec::<DirEntry>::new();
    let paths: ReadDir = fs::read_dir("./").unwrap();
    get_files_new(&mut files, paths);
    println!("{}", files.len());
    // let paths = fs::read_dir("./").unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
}

fn get_files_old(files: &mut Vec<DirEntry>, paths: ReadDir) -> () {
    for path in paths {
        match fs::read_dir(path.as_ref().unwrap().path()) {
            Ok(_) => {
                get_files_old(
                    files,
                    fs::read_dir(path.unwrap().path().display().to_string()).unwrap(),
                );
            }
            Err(_) => {
                files.push(path.unwrap());
            }
        }
    }
}

fn get_files_new(files: &mut Vec<DirEntry>, paths: ReadDir) -> () {
    for path in paths {
        let p = fs::read_dir(path.as_ref().unwrap().path());
        match p {
            Ok(_) => {
                get_files_new(files, p.unwrap());
            }
            Err(_) => {
                files.push(path.unwrap());
            }
        }
    }
}
