mod alamo_movies;
use crate::alamo_movies::cinema::Cinema;

#[macro_use] extern crate lazy_static;
extern crate regex;

fn main() {
    // first, read the file into a string
    let path = "0002.calendar.json";

    let cinema = Cinema::from_calendar_file(&path).expect("cannot load file");

    println!("cinema: {} / {} - {}", cinema.name, cinema.id, cinema.slug);
    println!("market: {} / {} - {}", cinema.market.name, cinema.market.id, cinema.market.slug);

    println!("Movies:");

    for movie in cinema.films.iter() {
        println!("  - {} ({})", movie.name, movie.rating);
    }
}

