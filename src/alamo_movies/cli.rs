use super::cinema::Cinema;
use super::film::Film;
use super::db;
use super::error::{NoCalendarFile, ExpiredCalendarFile};
use super::printer;

use std::process::exit;
use std::path::PathBuf;
use std::fs;
use std::error::Error;

use chrono::{DateTime, Utc};
use clap::{ArgMatches};
use rayon::prelude::*;

pub fn subcommand_films(matches: &ArgMatches) {
    let cinema_id = matches.value_of("cinema_id").unwrap();
    let cinema_id = Cinema::to_cinema_id(cinema_id).unwrap();

    let as_json = matches.is_present("json");

    let films = if let Some(film_type) = matches.value_of("type") {
        filtered_films_for(&cinema_id, film_type)
    } else {
        films_for(&cinema_id)
    };

    if as_json {
        printer::json_list_films(&films);
    } else {
        printer::list_films(&films);
    }
}

pub fn subcommand_cinema(matches: &ArgMatches) {
    let as_json = matches.is_present("json");

    match matches.value_of("cinema_id") {
        Some(cinema_id) => {
            // the user passed a cinema ID
            // so find that cinema and print it.
            let cinema_id = Cinema::to_cinema_id(&cinema_id).unwrap();
            let (cinema, _films) = load_or_sync_cinema_for_id(&cinema_id).expect("Failed to load cinema file.");

            if as_json {
                printer::json_cinema_info(&cinema);
            } else {
                printer::cinema_info(&cinema);
            }
        },
        None => {
            // the user did not pass a cinema ID
            // so print a list of all cinemas (with other args we got)
            let cinemas = get_cinema_list(matches);

            if as_json {
                printer::json_list_cinemas(&cinemas);
            } else {
                printer::list_cinemas(&cinemas);
            }
        }
    }
}

pub fn subcommand_get(matches: &ArgMatches) {
    let cinema_id = matches.value_of("cinema_id").unwrap();
    let cinema_id = Cinema::to_cinema_id(cinema_id).unwrap();

    if let Ok(_) = Cinema::sync_file(&cinema_id) {
        let path = db::calendar_path_for_cinema_id(&cinema_id);
        let (cinema, _films) = Cinema::from_calendar_file(path.to_str().unwrap()).expect("cannot load file");

        eprintln!("Synced {} {}", cinema.id, cinema.name);
    } else {
        panic!("Error");
    }
}

pub fn subcommand_get_all(matches: &ArgMatches) {
    let cinema_ids =
        if matches.is_present("update-only") {
            // only update the local files
            let path = db::base_directory_path();

            if ! path.is_dir() {
                eprintln!("No local cinema data to update.");
                return;
            }

            db::list_cinema_ids(path)
        } else {
            Cinema::list()
                .iter()
                .map(|c| c.id.clone())
                .collect()
        };

    let mut error_count = 0;

    for cinema_id in cinema_ids.iter() {
        error_count = error_count + match Cinema::sync_file(cinema_id) {
            Err(error) => {
                eprintln!("Failed to sync cinema {}: {}", cinema_id, error);
                1
            },
            Ok((cinema, _films)) => {
                eprintln!("Synced cinema {} {}", cinema.id, cinema.name);
                0
            },
        }
    }

    if error_count > 0 {
        exit(1);
    }
}

// XXX this should be a Result and not exit.
fn load_or_sync_cinema_for_id(cinema_id: &str) -> Option<(Cinema, Vec<Film>)> {
    let path = db::calendar_path_for_cinema_id(cinema_id);

    if let Err(_) = check_local_file(&path) {
        match Cinema::sync_file(cinema_id) {
            Err(error) => {
                eprintln!("Failed to download cinema data for cinema with ID {}: {}", cinema_id, error);
                eprintln!("Is this a valid cinema ID?");
                exit(1);
            },
            _ => eprintln!("Synced file for cinema via API."),
        }
    }

    match Cinema::from_calendar_file(path.to_str().unwrap()) {
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(1);
        },
        Ok(result) => Some(result),
    }
}

fn check_local_file(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // if there's no file, then it's no good
    if ! path.is_file() {
        return Err(Box::new(NoCalendarFile::from_path(path.to_str().unwrap())));
    }

    // if the file is expired, then it's no good
    let contents = fs::read_to_string(path).expect("Failed to read file");
    let v: serde_json::Value = serde_json::from_str(&contents)?;

    let date_time = String::from(v["Calendar"]["FeedGenerated"].as_str().unwrap()) + "Z";

    let parsed_date = DateTime::parse_from_rfc3339(&date_time)?;

    let now = Utc::now();

    let duration = now.signed_duration_since(parsed_date);

    // check the duration. make sure it's not older than 24 hours.
    if duration.num_hours() > 24 {
        return Err(Box::new(ExpiredCalendarFile::from_date_time(&date_time)));
    }

    Ok(())
}

fn films_for(cinema_id: &str) -> Vec<Film> {
    match load_or_sync_cinema_for_id(cinema_id) {
        Some((_cinema, mut films)) => {
            // list it out
            films.sort_by(|a,b| a.name.cmp(&b.name));

            films
        },
        None => {
            eprintln!("Failed to load cinema file.");
            vec![]
        }
    }
}

fn filtered_films_for(cinema_id: &str, film_type: &str) -> Vec<Film> {
    match load_or_sync_cinema_for_id(cinema_id) {
        Some((_cinema, mut films)) => {
            // list it out
            films.sort_by(|a,b| a.name.cmp(&b.name));

            films.iter()
                .filter(|f| f.show_type.to_lowercase() == film_type.to_lowercase() )
                .cloned()
                .collect()
        },
        None => {
            eprintln!("Failed to load cinema file.");
            vec![]
        },
    }
}

fn get_cinema_list(matches: &ArgMatches) -> Vec<Cinema> {
    if matches.is_present("local") {
        let db_path = db::base_directory_path();

        if ! db_path.is_dir() {
            return vec![];
        }

        let cinema_ids = db::list_cinema_ids(db_path);

        let mut cinemas: Vec<Cinema> =
            cinema_ids
                .par_iter()
                .map(|cinema_id| {
                    let (cinema, _films) = load_or_sync_cinema_for_id(&cinema_id).expect("Failed to load cinema file.");

                    cinema
                })
                .collect();

        cinemas.sort_by(|a, b| a.id.cmp(&b.id));

        cinemas
    } else {
        // print out the built-in cinema list
        Cinema::list().to_vec()
    }
}

