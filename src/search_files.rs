use super::MATCHED_FILES;
use super::SEARCHED_SIZE;
use super::TOTAL_SIZE;
use std::fs;
use std::fs::ReadDir;
use std::io::{stdout, Write};
use std::path::PathBuf;

pub fn loop_files(target: &str, paths: ReadDir) -> () {
    let mut stdout = stdout();
    for path in paths {
        let p: PathBuf = path.as_ref().unwrap().path();
        let maybe_dir = fs::read_dir(&p);
        match maybe_dir {
            Ok(_) => {
                loop_files(target, maybe_dir.unwrap());
            }
            Err(_) => {
                // this is a file
                unsafe {
                    SEARCHED_SIZE += &p.metadata().unwrap().len();
                    print!("\rProcessing {}%", (SEARCHED_SIZE * 100) / (TOTAL_SIZE));
                    stdout.flush().unwrap();
                }
                search_file(target, p);
            }
        }
    }
}

pub fn search_file(target: &str, path: PathBuf) {
    let chars = match fs::read_to_string(&path) {
        //an error here would only occur when the files isn't utf8,
        //meaning the text user searched isn't valid/they aren't searching this file so returning is valid
        Ok(s) => s.chars().collect::<Vec<char>>().into_iter(),
        Err(_) => return,
    };

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
