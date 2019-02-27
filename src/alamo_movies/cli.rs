use super::cinema::Cinema;
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
            let file_path = db::calendar_path_for_cinema(cinema_id);
            print_cinema_info_for_file(file_path.to_str().unwrap());
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

fn list_films_for(cinema_id: &str) {
    // first, read the file into a string
    let path = db::calendar_path_for_cinema(cinema_id);

    // if the file does not exist, then download it.
    if ! path.is_file() {
        match Cinema::sync_file(cinema_id) {
            Err(_) => panic!("Failed to get cinema file for id: {}", cinema_id),
            _ => eprintln!("Fetched new file for id: {}", cinema_id),
        }
    }

    let (_cinema, films) = Cinema::from_calendar_file(path.to_str().unwrap()).expect("cannot load file");

    // list it out
    for movie in films.iter() {
        println!("{}", movie.name);
    }
}

fn print_cinema_info_for_file(path: &str) {
    let (cinema, _films) = Cinema::from_calendar_file(path).expect("cannot load file");

    println!("{} {} ({})", cinema.id, cinema.name, cinema.market.name);
}

fn print_cinema_list(matches: &ArgMatches) {

    let local_only: bool = matches.occurrences_of("local") > 0;

    if local_only {
        let db_path = db::base_directory();

        for file in db::list_cinema_files(db_path) {
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
