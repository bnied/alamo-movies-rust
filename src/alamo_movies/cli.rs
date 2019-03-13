use super::cinema::Cinema;
use super::film::Film;
use super::db;
use super::error::{NoCalendarFile, ExpiredCalendarFile};
use super::printer;
use super::printer::Format;

use std::path::PathBuf;
use std::fs;
use std::error::Error;

use chrono::{DateTime, Utc};
use clap::{ArgMatches};
use rayon::prelude::*;

pub fn subcommand_films(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let cinema_id = matches.value_of("cinema_id").unwrap();
    let cinema_id = Cinema::to_cinema_id(cinema_id).unwrap();

    let format = format_for_match(matches);

    let films = if let Some(film_type) = matches.value_of("type") {
        filtered_films_for(&cinema_id, film_type)?
    } else {
        films_for(&cinema_id)?
    };

    printer::list_films(&films, &format);

    Ok(())
}

pub fn subcommand_cinema(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let format = format_for_match(matches);

    match matches.value_of("cinema_id") {
        Some(cinema_id) => {
            // the user passed a cinema ID
            // so find that cinema and print it.
            let cinema_id = Cinema::to_cinema_id(&cinema_id).unwrap();
            let (cinema, _films) = load_or_sync_cinema_for_id(&cinema_id)?;

            printer::cinema_info(&cinema, &format);
        },
        None => {
            // the user did not pass a cinema ID
            // so print a list of all cinemas (with other args we got)
            let cinemas = get_cinema_list(matches)?;

            printer::list_cinemas(&cinemas, &format);
        }
    }

    Ok(())
}

pub fn subcommand_get(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let cinema_id = matches.value_of("cinema_id").unwrap();
    let cinema_id = Cinema::to_cinema_id(cinema_id).unwrap();

    Cinema::sync_file(&cinema_id)?;

    let path = db::calendar_path_for_cinema_id(&cinema_id);
    let (cinema, _films) = Cinema::from_calendar_file(&path)?;

    eprintln!("Synced {} {}", cinema.id, cinema.name);

    Ok(())
}

pub fn subcommand_get_all(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let cinema_ids =
        if matches.is_present("update-only") {
            // only update the local files
            let path = db::base_directory_path();

            if ! path.is_dir() {
                eprintln!("No local cinema data to update.");
                return Ok(());
            }

            db::list_cinema_ids(path)
        } else {
            Cinema::list()
                .iter()
                .map(|c| c.id.clone())
                .collect()
        };

    for cinema_id in cinema_ids.iter() {
        match Cinema::sync_file(cinema_id) {
            Err(error) => {
                eprintln!("Failed to sync cinema {}: {}", cinema_id, error);
                return Err(error);
            },
            Ok((cinema, _films)) => {
                eprintln!("Synced cinema {} {}", cinema.id, cinema.name);
            },
        }
    }

    Ok(())
}

fn format_for_match(matches: &ArgMatches) -> Format {
    if matches.is_present("json") {
        Format::Json
    } else {
        Format::Text
    }
}

fn load_or_sync_cinema_for_id(cinema_id: &str) -> Result<(Cinema, Vec<Film>), Box<dyn Error>> {
    let path = db::calendar_path_for_cinema_id(cinema_id);

    if let Err(_) = check_local_file(&path) {
        match Cinema::sync_file(cinema_id) {
            Err(error) => {
                eprintln!("Failed to download cinema data for cinema with ID {}: {}", cinema_id, error);
                eprintln!("Is this a valid cinema ID?");
                return Err(error);
            },
            _ => eprintln!("Synced file for cinema via API."),
        }
    }

    Ok(Cinema::from_calendar_file(&path)?)
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

fn films_for(cinema_id: &str) -> Result<Vec<Film>, Box<dyn Error>> {
    let (_cinema, mut films) = load_or_sync_cinema_for_id(cinema_id)?;

    // list it out
    films.sort_by(|a,b| a.name.cmp(&b.name));

    Ok(films)
}

fn filtered_films_for(cinema_id: &str, film_type: &str) -> Result<Vec<Film>, Box<dyn Error>> {
    let (_cinema, mut films) = load_or_sync_cinema_for_id(cinema_id)?;

    // list it out
    films.sort_by(|a,b| a.name.cmp(&b.name));

    Ok(films.iter()
        .filter(|f| f.show_type.to_lowercase() == film_type.to_lowercase() )
        .cloned()
        .collect()
        )
}

fn get_cinema_list(matches: &ArgMatches) -> Result<Vec<Cinema>, Box<dyn Error>> {
    if matches.is_present("local") {
        let db_path = db::base_directory_path();

        if ! db_path.is_dir() {
            return Ok(vec![]);
        }

        let cinema_ids = db::list_cinema_ids(db_path);

        let mut cinemas: Vec<Cinema> =
            cinema_ids
                .par_iter()
                .map(|cinema_id| {
                    let (cinema, _films) = load_or_sync_cinema_for_id(&cinema_id).expect("Failed to get cinema");
                    cinema
                })
                .collect();

        cinemas.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(cinemas)
    } else {
        // print out the built-in cinema list
        Ok(Cinema::list().to_vec())
    }
}

