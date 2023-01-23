use std::fs;
use std::fs::ReadDir;
use std::io;

pub fn parse_args(args: Vec<String>) -> (String, String, io::Result<ReadDir>) {
    if args.len() == 1 {
        panic!("No directory or file given");
    }
    if args.len() == 2 {
        panic!("Did not give a target phrase or word");
    }

    let path: &str = args.get(1).unwrap();
    let target: String = args.get(2).unwrap().replace("\\n", "\n").into();

    if !std::path::Path::new(path).exists() {
        panic!("Location {}, doesn't exist", path);
    }

    let maybe_dir: io::Result<ReadDir> = fs::read_dir(path);

    (path.to_string(), target.to_string(), maybe_dir)
}
