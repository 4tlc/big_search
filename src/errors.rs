use std::fmt;
use std::path::PathBuf;

pub enum CustomError {
    FolderNotFound { cause: PathBuf },
    InvalidFlag { cause: char },
    NotEnoughArguments { cause: usize },
    NoFlagGiven,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            CustomError::FolderNotFound { cause } => {
                write!(f, "Path given {:?} doesn't exst", cause)
            }
            CustomError::NotEnoughArguments { cause } => {
                write!(
                    f,
                    "Not enough arguments given, you gave {}, bs requires 2 (path, target)",
                    cause
                )
            }
            CustomError::InvalidFlag { cause } => {
                write!(f, "Flag {} is not valid", cause)
            }
            CustomError::NoFlagGiven => {
                write!(f, "No flag given after '-' character")
            }
        }
    }
}
