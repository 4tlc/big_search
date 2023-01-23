use crate::MATCHED_FILES;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;

pub fn loop_files(target: &str, paths: ReadDir) -> () {
    for path in paths {
        let p: PathBuf = path.as_ref().unwrap().path();
        let maybe_dir = fs::read_dir(&p);
        match maybe_dir {
            Ok(_) => {
                loop_files(target, maybe_dir.unwrap());
            }
            Err(_) => {
                // this is a file
                search_file(target, p);
            }
        }
    }
}

pub fn search_file(target: &str, path: PathBuf) {
    let chars: std::vec::IntoIter<char> = fs::read_to_string(&path)
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .into_iter();

    // this will always be the length of the target
    // this will be checked against target
    let mut window = String::new();
    if chars.len() < target.len() {
        return;
    }

    for char in chars {
        if window.len() < target.len() {
            window.push(char);
            continue;
        }
        if add_if_match(target, window.as_ref(), &path) {
            return;
        }
        window.remove(0);
        window.push(char);
    }
    // in case their is a match at the end of file
    add_if_match(target, window.as_ref(), &path);
}

fn add_if_match(target: &str, buffer: &str, path: &PathBuf) -> bool {
    if target == buffer {
        unsafe {
            MATCHED_FILES.push(path.display().to_string());
        }
        return true;
    }
    return false;
}
