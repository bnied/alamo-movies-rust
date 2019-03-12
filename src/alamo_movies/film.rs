use std::error::Error;
use std::collections::HashSet;

use super::film_title::FilmTitle;

#[derive(Clone)]
pub struct Film {
    pub id: String,
    pub name: String,
    pub year: String,
    pub rating: String,
    pub runtime: String,
    pub slug: String,
    pub show_type: String, // Things like "Terror Tuesday" or "Movie Party"
    pub suffix: String, // like "in 35mm"
}

impl Film {
    pub fn from_json(json: &serde_json::Value) -> Result<Film, Box<dyn Error>> {
        let id = id_from(json).unwrap();
        let film_title = name_from(json).unwrap();
        let year = year_from(json).unwrap();
        let rating = rating_from(json).unwrap();
        let runtime = runtime_from(json).unwrap();
        let slug = slug_from(json).unwrap();

        Ok(Film {
            id: id.to_string(),
            name: film_title.title.to_string(),
            year: year.to_string(),
            rating: rating.to_string(),
            runtime: runtime.to_string(),
            slug: slug.to_string(),
            show_type: film_title.show_type.to_string(),
            suffix: film_title.suffix.to_string(),
        })
    }

    pub fn films_from_calendar(json: &serde_json::Value) -> Result<Vec<Film>, Box<dyn Error>> {
        let mut results: Vec<Film> = Vec::new();

        let mut film_ids = HashSet::new();

        // iterate over the Months > Weeks > Days > Films
        for month in json["Months"].as_array().unwrap().iter() {
            for week in month["Weeks"].as_array().unwrap().iter() {
                for day in week["Days"].as_array().unwrap().iter() {
                    match day["Films"].as_array() {
                        None => {},
                        Some(films) => {
                            for film_json in films {
                                let film = Film::from_json(film_json)?;

                                if film_ids.insert(film.id.to_string()) {
                                    results.push(film);
                                }

                            }
                        },
                    };
                }
            }
        }

        Ok(results)
    }
}



fn id_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmId"].as_str()
}

fn name_from(data: &serde_json::Value) -> Option<FilmTitle> {
    let name = data["FilmName"].as_str()?;
    FilmTitle::parse(name)
}

fn year_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmYear"].as_str()
}

fn rating_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmRating"].as_str()
}

fn runtime_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmRuntime"].as_str()
}

fn slug_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmSlug"].as_str()
}

