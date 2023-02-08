use std::panic::set_hook;
use std::path::PathBuf;

pub fn parse_args(args: Vec<String>) -> (PathBuf, String, bool) {
    if args.len() < 3 {
        set_hook(Box::new(|_info| {
            println!("Error: Either not given a search location or not given a target phrase");
        }));
        panic!();
    }

    let first: &str = args.get(1).unwrap();

    // if its not a path then check if its a flag
    let mut calc_size: bool = true;
    let mut path: Option<&str> = None;
    let mut target: Option<String> = None;
    if !std::path::Path::new(first).exists() {
        // loop through all the flags
        for arg in args.iter().skip(1) {
            if arg.chars().nth(0).unwrap() == '-' {
                match arg.chars().nth(1) {
                    Some('n') => calc_size = false,
                    Some(c) => {
                        set_hook(Box::new(move |_| {
                            println!("Error: {} is not a valid flag", c);
                        }));
                        panic!();
                    }
                    _ => {
                        set_hook(Box::new(|_info| {
                            println!("Error: No flag character given after '-'");
                        }));
                        panic!();
                    }
                }
            } else {
                if path == None {
                    path = Some(arg);
                } else {
                    target = Some(arg.to_string());
                }
            }
        }
    } else {
        path = Some(first);
        target = args.get(2).unwrap().replace("\\n", "\n").into();
    }
    if path == None || target == None {
        set_hook(Box::new(|_info| {
            println!("Error: Either not given a search location or not given a target phrase");
        }));
        panic!();
    }

    let path = PathBuf::from(path.unwrap());
    if !path.exists() {
        set_hook(Box::new(move |_info| {
            println!("Error: The Given path, {:?} doesn't exist", path);
        }));
        panic!();
    }
    (path, target.unwrap().to_string(), calc_size)
}
