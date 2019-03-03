use std::fmt;
use std::error;
use chrono::{DateTime};

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

#[derive(Debug, Clone)]
pub struct NoCalendarFile {
    path: String,
}

impl NoCalendarFile {
    pub fn from_path(path: &str) -> NoCalendarFile {
        NoCalendarFile {
            path: String::from(path),
        }
    }
}

impl fmt::Display for NoCalendarFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File does not exist: {}", self.path)
    } 

}

impl error::Error for NoCalendarFile {
    fn description(&self) -> &str {
        "file does not exist"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct ExpiredCalendarFile {
    pub created_at: String,
}

impl ExpiredCalendarFile {
    pub fn from_date_time(created_at: &str) -> ExpiredCalendarFile {
        ExpiredCalendarFile {
           created_at: String::from(created_at), 
        }
    }
}

impl fmt::Display for ExpiredCalendarFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Calendar data is expired: {}", self.created_at)
    } 

}

impl error::Error for ExpiredCalendarFile {
    fn description(&self) -> &str {
        "calendar data is expired"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
