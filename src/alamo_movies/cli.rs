use super::cinema::Cinema;
use super::film::Film;
use super::db;

use clap::{ArgMatches};

pub fn subcommand_films(matches: &ArgMatches) {
    let cinema_id = matches.value_of("cinema_id").unwrap();

    list_films_for(cinema_id);
}

pub fn subcommand_cinema(matches: &ArgMatches) {
    match matches.value_of("cinema_id") {
        Some(cinema_id) => { 
            // the user passed a cinema ID
            // so find that cinema and print it.
            let (cinema, _films) = load_or_sync_cinema_for_id(&cinema_id).expect("Failed to load cinema file.");
            print_cinema_info(&cinema);
        },
        None =>
            // the user did not pass a cinema ID
            // so print a list of all cinemas (with other args we got)
            print_cinema_list(matches),
    }
}

pub fn subcommand_get(matches: &ArgMatches) {
    let cinema_id = matches.value_of("cinema_id").unwrap();

    if let Ok(_) = Cinema::sync_file(cinema_id) {
        let path = db::calendar_path_for_cinema(cinema_id);
        let (cinema, _films) = Cinema::from_calendar_file(path.to_str().unwrap()).expect("cannot load file");

        println!("Synced {} {}", cinema.id, cinema.name);
    } else {
        panic!("Error");
    }
}

fn load_or_sync_cinema_for_id(cinema_id: &str) -> Option<(Cinema, Vec<Film>)> {
    let path = db::calendar_path_for_cinema(cinema_id);

    // if the file does not exist, then download it.
    if ! path.is_file() {
        match Cinema::sync_file(cinema_id) {
            Err(_) => return None,
            _ => eprintln!("Synced file for cinema via API."),
        }
    }

    Some(Cinema::from_calendar_file(path.to_str().unwrap()).expect("cannot load file"))
}

fn list_films_for(cinema_id: &str) {
    match load_or_sync_cinema_for_id(cinema_id) {
        Some((_cinema, films)) => {
            // list it out
            for film in films.iter() {
                println!("{}", film.name);
            }
        },
        None => {
            eprintln!("Failed to load cinema file.");
            std::process::exit(1);
        }
    }
}

fn print_cinema_info(cinema: &Cinema) {
    println!("{cinema_id} [{cinema_slug}] {cinema_name} ({market})",
        cinema_id = cinema.id,
        cinema_slug = cinema.slug,
        cinema_name = cinema.name,
        market = cinema.market.name,
    );
}

fn print_cinema_list(matches: &ArgMatches) {

    let local_only: bool = matches.occurrences_of("local") > 0;

    if local_only {
        let db_path = db::base_directory();

        for cinema_id in db::list_cinema_ids(db_path) {
            let (cinema, _films) = load_or_sync_cinema_for_id(&cinema_id).expect("Failed to load cinema file.");
            print_cinema_info(&cinema);
        }
    } else {
        // print out the built-in cinema list
        let cinemas = Cinema::list();

        for cinema in cinemas.iter() {
            print_cinema_info(cinema);
        }
    }
}
