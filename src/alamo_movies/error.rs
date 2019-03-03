use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct InvalidCinemaFile {
    path: String,
}

impl InvalidCinemaFile {
    pub fn for_path(path: &str) -> InvalidCinemaFile {
        InvalidCinemaFile {
            path: String::from(path),
        }
    }
}

impl fmt::Display for InvalidCinemaFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid cinema file: {}", self.path)
    } 

}

impl error::Error for InvalidCinemaFile {
    fn description(&self) -> &str {
        "cinema id does not point to a valid cinema"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct InvalidCinemaData;

impl fmt::Display for InvalidCinemaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid cinema data.")
    } 

}

impl error::Error for InvalidCinemaData {
    fn description(&self) -> &str {
        "data does not represent a valid cinema and films"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
