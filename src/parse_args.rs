use crate::errors::CustomError;
// use std::error::Error;
use std::path::PathBuf;

pub fn parse_args(args: Vec<String>) -> Result<(PathBuf, String, bool), CustomError> {
    if args.len() < 3 {
        return Err(CustomError::NotEnoughArguments {
            cause: args.len() - 1,
        }); // subtract one cause system adds one
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
                    Some(c) => return Err(CustomError::InvalidFlag { cause: c }),
                    _ => return Err(CustomError::NoFlagGiven),
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
        return Err(CustomError::NotEnoughArguments {
            cause: args.len() - 1, //subtract one cause system gives one
        });
    }

    let path = PathBuf::from(path.unwrap());
    if !path.exists() {
        return Err(CustomError::FolderNotFound { cause: path });
    }
    return Ok((path, target.unwrap().to_string(), calc_size));
}
