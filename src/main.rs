mod search_files;
use crate::search_files::{loop_files, search_file};
mod parse_args;
use crate::parse_args::parse_args;
use std::env;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::io;
use std::path::PathBuf;

pub static mut MATCHED_FILES: Vec<String> = Vec::<String>::new();
pub static mut SEARCHED_SIZE: u64 = 0;
static mut SIZE: u64 = 0;

fn main() {
    let (path, target, maybe_dir) = parse_args(env::args().collect());
    let mut size: u64 = 0;
    match maybe_dir {
        Ok(_) => search_size(PathBuf::from(&path)),
        Err(_) => unsafe { SIZE = fs::metadata(&path).unwrap().len() },
    };
    // size = dir_size(path).unwrap();
    // println!("{}", size);
    begin_status();
    match maybe_dir {
        Ok(_) => {
            loop_files(&target, maybe_dir.unwrap());
        }
        Err(_) => search_file(&target, PathBuf::from(path)),
    }
    unsafe {
        println!("{:?}", MATCHED_FILES);
        println!("Mine: {}", SIZE);
    };
}

fn begin_status() {}

fn search_size(paths: PathBuf) {
    fn ss(dir: fs::ReadDir) {
        for path in dir {
            let p: PathBuf = path.as_ref().unwrap().path();
            let maybe_dir = fs::read_dir(&p);
            match maybe_dir {
                Ok(_) => {
                    ss(maybe_dir.unwrap());
                }
                Err(_) => unsafe { SIZE += fs::metadata(p).unwrap().len() },
            }
        }
    }
    ss(fs::read_dir(paths).unwrap());
}

// From: https://gitlab.com/users/Boiethios/projects | https://stackoverflow.com/questions/60041710/how-to-check-directory-size
fn dir_size(path: impl Into<PathBuf>) -> io::Result<u64> {
    fn dir_size(mut dir: fs::ReadDir) -> io::Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => dir_size(fs::read_dir(file.path())?)?,
                data => data.len(),
            };
            Ok(acc + size)
        })
    }
    dir_size(fs::read_dir(path.into())?)
}
