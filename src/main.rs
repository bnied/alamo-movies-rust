mod alamo_movies;

#[macro_use] extern crate lazy_static;
extern crate regex;

extern crate clap;
use clap::{Arg, App, SubCommand};

use crate::alamo_movies::cli;

fn main() {
    let matches = App::new("adc")
        .version("0.1.0")
        .author("Spike Grobstein <me@spike.cx>")
        .about("Query the Alamo Drafthouse schedule to get lists of upcoming films playing in theaters.")
        .subcommand(SubCommand::with_name("films")
                    .about("List films for the given theater")
                    .arg(Arg::with_name("cinema_id")
                         .help("The ID of the cinema from which to list upcoming films.")
                         .required(true)
                        )
                    .arg(Arg::with_name("type")
                         .help("Only list films of this type")
                         .takes_value(true)
                         .long("type")
                         .value_name("TYPE")
                         )
                    )
        .subcommand(SubCommand::with_name("cinema")
                    .about("List available cinemas.")
                    .alias("cinemas")
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
                    .alias("sync")
                    .arg(Arg::with_name("cinema_id")
                         .help("The ID of the cinema to fetch")
                         .required(true)
                         )
                    )
        .subcommand(SubCommand::with_name("get-all")
                    .about("Update all local cinema data for every cinema")
                    .alias("sync-all")
                    .arg(Arg::with_name("update-only")
                         .help("Only update existing local cinema data")
                         .long("update-only")
                         .takes_value(false)
                         )
                    )
        .after_help("adc is in no way affiliated with the Alamo Drafthouse Cinemas.\n\
                     I'm just a huge fan.")
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("films") {
        cli::subcommand_films(matches);
    } else if let Some(matches) = matches.subcommand_matches("cinema") {
        cli::subcommand_cinema(matches);
    } else if let Some(matches) = matches.subcommand_matches("get") {
        cli::subcommand_get(matches);
    } else if let Some(matches) = matches.subcommand_matches("get-all") {
        cli::subcommand_get_all(matches);
    }
}



