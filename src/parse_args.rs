use std::path::PathBuf;

pub fn parse_args(args: Vec<String>) -> (PathBuf, String, bool) {
    if args.len() < 3 {
        panic!("Either not given a search location or not given a target phrase");
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
                    Some(c) => panic!("{} is not a valid flag", c),
                    _ => panic!("No character given for flag"),
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
        panic!("Either not given a search location or not given a target phrase");
    }

    (
        PathBuf::from(path.unwrap()),
        target.unwrap().to_string(),
        calc_size,
    )
}
