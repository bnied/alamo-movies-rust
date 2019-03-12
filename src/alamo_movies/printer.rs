use super::cinema::Cinema;
use super::film::Film;

pub fn film_info(film: &Film) {
    println!("{}", film.name);
}

pub fn film_list(films: &Vec<Film>) {
    for film in films.iter() {
        film_info(film);
    }
}

pub fn cinema_info(cinema: &Cinema) {
    println!("{cinema_id} [{cinema_slug}] {cinema_name} ({market})",
        cinema_id = cinema.id,
        cinema_slug = cinema.slug,
        cinema_name = cinema.name,
        market = cinema.market.name,
    );
}

pub fn cinema_list(cinemas: &Vec<Cinema>) {
    for cinema in cinemas {
        cinema_info(cinema);
    }
}
