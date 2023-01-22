use std::fs;
use std::fs::{DirEntry, ReadDir};

static mut FILES: Vec<DirEntry> = Vec::<DirEntry>::new();

fn main() {
    let paths: ReadDir = fs::read_dir("./").unwrap();
    get_files(paths);
    // let paths = fs::read_dir("./").unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
}

fn get_files(paths: ReadDir) -> () {
    for path in paths {
        match fs::read_dir(path.as_ref().unwrap().path()) {
            Ok(_) => get_files(fs::read_dir(path.unwrap().path().display().to_string()).unwrap()),
            Err(_) => println!("file: {}", path.unwrap().path().display()),
        }
    }
}
