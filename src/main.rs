mod alamo_movies;
use crate::alamo_movies::cinema::Cinema;

#[macro_use] extern crate lazy_static;
extern crate regex;

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Alamo Movies")
        .version("0.1.0")
        .author("Spike Grobstein <me@spike.cx>")
        .about("Query the Alamo Drafthouse schedule to get lists of upcoming films playing in theaters.")
        .subcommand(SubCommand::with_name("films")
                    .about("List films for the given theater")
                    .arg(Arg::with_name("cinema_id")
                         .help("The ID of the cinema from which to list upcoming films.")
                         .required(true))
                    )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("films") {
        let cinema_id = matches.value_of("cinema_id").unwrap();
        println!("Listing films!");

        // first, read the file into a string
        let path = Cinema::get_file_path_for(cinema_id);

        println!("Opening file: {}", path);

        let cinema = Cinema::from_calendar_file(&path).expect("cannot load file");

        println!("cinema: {} / {} - {}", cinema.name, cinema.id, cinema.slug);
        println!("market: {} / {} - {}", cinema.market.name, cinema.market.id, cinema.market.slug);

        println!("Movies:");

        for movie in cinema.films.iter() {
            println!("  - {} ({})", movie.name, movie.rating);
        }
    };
    // let args: Vec<String> = env::args().collect();
    // let cinema_id = args.get(1).expect("Please supply a cinema_id");

}

