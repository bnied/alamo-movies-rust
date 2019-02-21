use std::fs;
use std::error::Error;

use std::env;
use std::path::{PathBuf};

use super::market::Market;
use super::film::Film;

pub struct Cinema {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub market: Market,
    pub films: Vec<Film>,
}

impl Cinema {
    pub fn from_calendar_file(path: &str) -> Result<Cinema, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;

        let v: serde_json::Value = serde_json::from_str(&contents)?;

        let data = &v["Calendar"]["Cinemas"][0];

        Cinema::from_calendar_json(data)
    }

    pub fn from_calendar_json(json: &serde_json::Value) -> Result<Cinema, Box<dyn Error>> {
        let name = cinema_name_from(json).unwrap();
        let id = cinema_id_from(json).unwrap();
        let slug = cinema_slug_from(json).unwrap();

        let market = Market::from_calendar_json(json)?;

        Ok(Cinema {
            id: id.to_string(),
            name: name.to_string(),
            slug: slug.to_string(),
            market,
            films: Film::films_from_calendar(json)?,
        })
    }

    /// Given a cinema ID,
    /// construct a path to a the json file in the db directory
    pub fn get_file_path_for(cinema_id: &str) -> String {
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

        String::from(db_path.to_str().unwrap())
    }
}

fn cinema_name_from(data: &serde_json::Value) -> Option<&str> {
    data["CinemaName"].as_str()
}

fn cinema_id_from(data: &serde_json::Value) -> Option<&str> {
    data["CinemaId"].as_str()
}

fn cinema_slug_from(data: &serde_json::Value) -> Option<&str> {
    data["CinemaSlug"].as_str()
}

