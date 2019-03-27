mod alamo_movies;
use crate::alamo_movies::cinema::Cinema;

#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

extern crate clap;
use clap::{Arg, App, SubCommand};

use std::fs;
use std::env;
use std::path::{PathBuf};

fn main() {
    let matches = App::new("Alamo Movies")
        .version("0.1.0")
        .author("Spike Grobstein <me@spike.cx>")
        .about("Query the Alamo Drafthouse schedule to get lists of upcoming films playing in theaters.")
        .subcommand(SubCommand::with_name("films")
                    .about("List films for the given theater")
                    .arg(Arg::with_name("cinema_id")
                         .help("The ID of the cinema from which to list upcoming films.")
                         .required(true)
                        )
                    )
        .subcommand(SubCommand::with_name("cinema")
                    .about("List available cinemas.")
                    .arg(Arg::with_name("local")
                         .help("Only print from local data")
                         .required(false)
                         .short("l")
                         .long("local")
                         .takes_value(false)
                         )
                    .arg(Arg::with_name("cinema_id")
                         .help("The ID of the cinema to get info about")
                         .required(false)
                         )
                    )
        .subcommand(SubCommand::with_name("get")
                    .about("Fetch the given cinema")
                    .arg(Arg::with_name("cinema_id")
                         .help("The ID of the cinema to fetch")
                         .required(true)
                         )
                    )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("films") {
        let cinema_id = matches.value_of("cinema_id").unwrap();

        list_films_for(cinema_id);
    } else if let Some(matches) = matches.subcommand_matches("cinema") {
        match matches.value_of("cinema_id") {
            Some(cinema_id) => 
                print_cinema_info_for(cinema_id),
            None =>
                print_cinema_list(matches),
        }
    } else if let Some(matches) = matches.subcommand_matches("get") {
        let cinema_id = matches.value_of("cinema_id").unwrap();

        if let Ok(_) = Cinema::sync_file(cinema_id) {
            let path = Cinema::get_file_path_for(cinema_id);
            let (cinema, _films) = Cinema::from_calendar_file(path.to_str().unwrap()).expect("cannot load file");

            println!("Synced {} {}", cinema.id, cinema.name);
        } else {
            panic!("Error");
        }
    }
}

fn list_films_for(cinema_id: &str) {
    // first, read the file into a string
    let path = Cinema::get_file_path_for(cinema_id);

    // if the file does not exist, then download it.
    if ! path.is_file() {
        match Cinema::sync_file(cinema_id) {
            Err(_) => panic!("Failed to get cinema file for id: {}", cinema_id),
            _ => eprintln!("Fetched new file for id: {}", cinema_id),
        }
    }

    let (cinema, films) = Cinema::from_calendar_file(path.to_str().unwrap()).expect("cannot load file");

    // list it out
    for movie in films.iter() {
        println!("{}", movie.name);
    }
}

fn print_cinema_info_for(cinema_id: &str) {
    let path = Cinema::get_file_path_for(cinema_id);

    print_cinema_info_for_file(path.to_str().unwrap());
}

fn print_cinema_info_for_file(path: &str) {
    let (cinema, _films) = Cinema::from_calendar_file(path).expect("cannot load file");

    println!("{} {} ({})", cinema.id, cinema.name, cinema.market.name);
}

fn print_cinema_list(matches: &clap::ArgMatches) {

    let local_only: bool = matches.occurrences_of("local") > 0;

    if local_only {
        let home_dir = match env::var("HOME") {
            Ok(home) => home,
            _ => String::from(""),
        };

        let mut db_path = PathBuf::from(home_dir);
        db_path = db_path
            .join(".alamo")
            .join("db");

        for file in get_cinema_files(db_path) {
            print_cinema_info_for_file(file.to_str().unwrap());
        }
    } else {
        // print out the built-in cinema list
        let cinemas = Cinema::list();

        for cinema in cinemas.iter() {
            println!("{} {} ({})", cinema.id, cinema.name, cinema.market.name);
        }
    }
}

fn get_cinema_files(path: PathBuf) -> Vec<PathBuf> {
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

fn is_calendar_file(path: PathBuf) -> bool {
    lazy_static! {
         static ref RE: Regex = Regex::new(r"\.calendar\.json$").unwrap();
    }

    RE.is_match(path.to_str().unwrap())
}
