use std::env;
use std::fs;
use std::fs::ReadDir;
use std::io;
use std::path::PathBuf;

static mut MATCHED_FILES: Vec<PathBuf> = Vec::<PathBuf>::new();

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("No directory or file given");
    }
    if args.len() == 2 {
        panic!("Did not give a target phrase or word");
    }

    let path: &str = args.get(1).unwrap();
    let target: &str = args.get(2).unwrap();

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
    let chars: Vec<char> = fs::read_to_string(&path).unwrap().chars().collect();
    let mut buffer: String = String::new();
    for char in chars {
        if char == '\n' {
            continue;
        }
        if char == ' ' {
            println!("{}", buffer);
            if check_phrase(target, buffer.as_str()) {
                unsafe {
                    // MATCHED_FILES.push(
                    //     path.into_os_string()
                    //         .into_string()
                    //         .unwrap()
                    //         .as_str()
                    //         .to_owned(),
                    // );
                    MATCHED_FILES.push(path);
                    return;
                }
            }
            buffer = String::new();
        }
        buffer.push(char);
    }
    println!("{}", buffer);
    if check_phrase(target, buffer.as_str()) {
        //if match at end of file
        unsafe {
            // MATCHED_FILES.push(
            //     path.into_os_string()
            //         .into_string()
            //         .unwrap()
            //         .as_str()
            //         .to_owned(),
            // );
            MATCHED_FILES.push(path);
        }
    }
}

fn check_phrase(target: &str, s: &str) -> bool {
    target == s
}
