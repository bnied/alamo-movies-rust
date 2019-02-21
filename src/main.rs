mod alamo_movies;
use crate::alamo_movies::cinema::Cinema;

use std::env;

#[macro_use] extern crate lazy_static;
extern crate regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cinema_id = args.get(1).expect("Please supply a cinema_id");

    // first, read the file into a string
    let mut path = String::from("./db/");
    path.push_str(cinema_id);
    path.push_str(".calendar.json");

    let cinema = Cinema::from_calendar_file(&path).expect("cannot load file");

    println!("cinema: {} / {} - {}", cinema.name, cinema.id, cinema.slug);
    println!("market: {} / {} - {}", cinema.market.name, cinema.market.id, cinema.market.slug);

    println!("Movies:");

    for movie in cinema.films.iter() {
        println!("  - {} ({})", movie.name, movie.rating);
    }
}

