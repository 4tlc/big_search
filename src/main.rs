use std::env;
use std::fs;
use std::fs::ReadDir;
use std::io;
use std::path::PathBuf;

static mut MATCHED_FILES: Vec<String> = Vec::<String>::new();

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("No directory or file given");
    }
    if args.len() == 2 {
        panic!("Did not give a target phrase or word");
    }

    let path: &str = args.get(1).unwrap();
    let cleaned: String = args.get(2).unwrap().replace("\\n", "\n").into();
    let target: &str = cleaned.as_str();

    println!("{:?}", target);

    if !std::path::Path::new(path).exists() {
        panic!("File given, {}, doesn't exist", path);
    }

    let maybe_dir: io::Result<ReadDir> = fs::read_dir(path);
    match maybe_dir {
        Ok(_) => {
            loop_files(target, maybe_dir.unwrap());
        }
        Err(_) => search_file(target, PathBuf::from(path)),
    }
    unsafe {
        println!("{:?}", MATCHED_FILES);
    };
}

fn loop_files(target: &str, paths: ReadDir) -> () {
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

fn search_file(target: &str, path: PathBuf) {
    // let chars: std::slice::Iter<'_, char> =
    //     fs::read_to_string(&path).unwrap().chars().collect::<Vec<chars>().iter();
    let chars: std::vec::IntoIter<char> = fs::read_to_string(&path)
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .into_iter();

    let mut window = String::new();
    if chars.len() < target.len() {
        return;
    }

    for char in chars {
        if window.len() < target.len() {
            window.push(char);
            continue;
        }
        add_if_match(target, window.as_ref(), &path);
        window.remove(0);
    }
    // for char in chars {
    //     if char == '\n' {
    //         continue;
    //     }
    //     if char == ' ' {
    //         println!("{}", buffer);
    //         if add_if_match(target, buffer.as_str(), &path) {
    //             return;
    //         }
    //         buffer = String::new();
    //     }
    //     buffer.push(char);
    // }
    // add_if_match(target, buffer.as_str(), &path); // if there is match at end of file
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
