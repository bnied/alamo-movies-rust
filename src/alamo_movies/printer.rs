use super::cinema::Cinema;
use super::film::Film;

pub enum Format {
    Text,
    Json,
}

/// given a vector of films, print them out accordingly
pub fn list_films(films: &Vec<Film>, format: &Format) {
    match format {
        Format::Json => json_list_films(films),
        Format::Text => {
            for film in films.iter() {
                film_info(film, format);
            }
        },
    }
}

pub fn list_cinemas(cinemas: &Vec<Cinema>, format: &Format) {
    match format {
        Format::Json => json_list_cinemas(cinemas),
        Format::Text => {
            for cinema in cinemas {
                cinema_info(cinema, format);
            }
        },
    }
}

pub fn film_info(film: &Film, _format: &Format) {
    println!("{}", film.name)
}

pub fn json_list_films(films: &Vec<Film>) {
    match serde_json::to_string(films) {
        Ok(json) => println!("{}", json),
        _ => panic!("whoops"),
    }
}

pub fn cinema_info(cinema: &Cinema, _format: &Format) {
    println!("{cinema_id} [{cinema_slug}] {cinema_name} ({market})",
        cinema_id = cinema.id,
        cinema_slug = cinema.slug,
        cinema_name = cinema.name,
        market = cinema.market.name,
    );
}

pub fn json_cinema_info(cinema: &Cinema) {
    match serde_json::to_string(cinema) {
        Ok(json) => println!("{}", json),
        _ => panic!("whoops"),
    }
}

pub fn json_list_cinemas(cinemas: &Vec<Cinema>) {
    match serde_json::to_string(cinemas) {
        Ok(json) => println!("{}", json),
        _ => panic!("whoops"),
    }
}

