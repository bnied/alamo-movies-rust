use std::env;
use std::path::PathBuf;
use std::fs;
use std::io::prelude::*;

use regex::Regex;

/// returns a list of all cinema files from the given path
pub fn list_cinema_files(path: PathBuf) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter(|entry| {
            match entry {
                Ok(entry) => !entry.path().is_dir() && is_calendar_file(entry.path()),
                _ => false,
            }
        })
        .map(|entry| {
            if let Ok(entry) = entry {
                entry.path()
            } else {
                panic!("This shouldn't happen")
            }
        })
        .collect()
}

/// private function that, given a path
/// returns whether it's a calendar file by inspecting the filename
fn is_calendar_file(path: PathBuf) -> bool {
    lazy_static! {
         static ref RE: Regex = Regex::new(r"\.calendar\.json$").unwrap();
    }

    RE.is_match(path.to_str().unwrap())
}

/// Given a cinema ID,
/// construct a path to a the json file in the db directory
pub fn calendar_path_for_cinema(cinema_id: &str) -> PathBuf {
    let home_dir = match env::var("HOME") {
        Ok(home) => home,
        _ => String::from(""),
    };

    // the db directory is ~/.alamo-movies/db 
    let mut filename = String::from(cinema_id);
    filename.push_str(".calendar.json");

    let mut db_path = PathBuf::from(home_dir);
    db_path = db_path
        .join(".alamo")
        .join("db")
        .join(filename);

    db_path
}

/// given the ID of the cinema and string data (from the web API)
/// write it to the spot
pub fn write_calendar_file(cinema_id: &str, data: &str) -> Result<(), Box<std::io::Error>> {
    let filepath = calendar_path_for_cinema(cinema_id);
    let mut file = fs::File::create(filepath)?;

    let result = file.write_all(data.as_bytes())?;

    Ok(result)
}
