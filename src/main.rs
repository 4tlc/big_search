mod search_files;
use crate::search_files::{loop_files, search_file};
mod parse_args;
use crate::parse_args::parse_args;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub static mut MATCHED_FILES: Vec<String> = Vec::<String>::new();
pub static mut SEARCHED_SIZE: u64 = 0;

fn main() {
    let (path, target) = parse_args(env::args().collect());
    let p = PathBuf::new();
    begin_status();
    let mut size: u64 = 0;
    size = search_size(PathBuf::from(&path));
    match std::fs::read_dir(PathBuf::from(&path)) {
        Ok(dir) => {
            loop_files(&target, dir);
        }
        Err(_) => {
            search_file(&target, PathBuf::from(path));
        }
    }
    unsafe {
        println!("{:?}", MATCHED_FILES);
        println!("Mine: {}", size);
    };
}

fn begin_status() {}

fn search_size(paths: PathBuf) -> u64 {
    fn ss(dir: fs::ReadDir) -> u64 {
        let mut size: u64 = 0;
        for path in dir {
            let p: PathBuf = path.as_ref().unwrap().path();
            match fs::read_dir(&p) {
                Ok(maybe_dir) => size += ss(maybe_dir),
                Err(_) => size += fs::metadata(p).unwrap().len(),
            }
        }
        size
    }
    match fs::read_dir(&paths) {
        Ok(dir) => ss(dir),
        Err(_) => fs::metadata(paths).unwrap().len(),
    }
}

// From: https://gitlab.com/users/Boiethios/projects | https://stackoverflow.com/questions/60041710/how-to-check-directory-size
fn dir_size(path: impl Into<PathBuf>) -> io::Result<u64> {
    fn ds(mut dir: fs::ReadDir) -> io::Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => ds(fs::read_dir(file.path())?)?,
                data => data.len(),
            };
            Ok(acc + size)
        })
    }
    ds(fs::read_dir(path.into())?)
}
