use std::env;
use std::fs;
use std::fs::ReadDir;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("No directory or file given");
    }

    let path: &str = args.get(1).unwrap();

    if !std::path::Path::new(path).exists() {
        panic!("File given, {}, doesn't exist", path);
    }

    let maybe_dir: io::Result<ReadDir> = fs::read_dir(path);
    match maybe_dir {
        Ok(_) => {
            search_files(maybe_dir.unwrap());
        }
        Err(_) => search_file(path),
    }
}

fn search_file(path: &str) {
    println!("{:?}", fs::read_to_string(path).unwrap());
}

fn search_files(paths: ReadDir) -> () {
    for path in paths {
        let p = path.as_ref().unwrap().path();
        let maybe_dir = fs::read_dir(&p);
        match maybe_dir {
            Ok(_) => {
                search_files(maybe_dir.unwrap());
            }
            Err(_) => {
                // this is a file
                println!("{:?}", fs::read_to_string(&p).unwrap());
            }
        }
    }
}
