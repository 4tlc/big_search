use std::path::PathBuf;

pub fn parse_args(args: Vec<String>) -> (PathBuf, String, bool) {
    if args.len() == 1 {
        panic!("No directory or file given");
    }
    if args.len() == 2 {
        panic!("Did not give a target phrase or word");
    }

    let first: &str = args.get(1).unwrap();

    // if its not a path then check if its a flag
    let mut calc_size: bool = true;
    let path: &str;
    let target: String;
    if !std::path::Path::new(first).exists() {
        if first == "-n" {
            calc_size = false;
            if args.len() == 4 {
                path = args.get(2).unwrap();
                target = args.get(3).unwrap().replace("\\n", "\n").into();
            } else {
                panic!("Not enough arguments given");
            }
        } else {
            panic!("Location {}, doesn't exist", first);
        }
    } else {
        path = first;
        target = args.get(2).unwrap().replace("\\n", "\n").into();
    }

    (PathBuf::from(path), target.to_string(), calc_size)
}
