use std::fs;
use std::error::Error;
use std::path::PathBuf;

use reqwest;
use regex::Regex;

use super::market::Market;
use super::film::Film;
use super::db;
use super::error::{InvalidCinemaData};

use serde::{Serialize};

#[derive(Clone, Serialize)]
pub struct Cinema {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub market: Market,
}

impl Cinema {
    pub fn list() -> [Cinema; 32] {
       [
            Cinema {
                id: String::from("0002"),
                name: String::from("Ritz"),
                slug: String::from("ritz"),
                market: Market {
                    id: String::from("0000"),
                    name: String::from("Austin, TX"),
                    slug: String::from("austin"),
                },
            },
            Cinema {
                id: String::from("0003"),
                name: String::from("Village"),
                slug: String::from("village"),
                market: Market {
                    id: String::from("0000"),
                    name: String::from("Austin, TX"),
                    slug: String::from("austin"),
                },
            },
            Cinema {
                id: String::from("0004"),
                name: String::from("South Lamar"),
                slug: String::from("south-lamar"),
                market: Market {
                    id: String::from("0000"),
                    name: String::from("Austin, TX"),
                    slug: String::from("austin"),
                },
            },
            Cinema {
                id: String::from("0006"),
                name: String::from("Slaughter Lane"),
                slug: String::from("slaughter-lane"),
                market: Market {
                    id: String::from("0000"),
                    name: String::from("Austin, TX"),
                    slug: String::from("austin"),
                },
            },
            Cinema {
                id: String::from("0007"),
                name: String::from("Lakeline"),
                slug: String::from("lakeline"),
                market: Market {
                    id: String::from("0000"),
                    name: String::from("Austin, TX"),
                    slug: String::from("austin"),
                },
            },
            Cinema {
                id: String::from("0008"),
                name: String::from("Mueller"),
                slug: String::from("mueller"),
                market: Market {
                    id: String::from("0000"),
                    name: String::from("Austin, TX"),
                    slug: String::from("austin"),
                },
            },
            Cinema {
                id: String::from("0106"),
                name: String::from("LaCenterra"),
                slug: String::from("lacenterra"),
                market: Market {
                    id: String::from("0100"),
                    name: String::from("Houston, TX"),
                    slug: String::from("houston"),
                },
            },
            Cinema {
                id: String::from("0201"),
                name: String::from("Westlakes"),
                slug: String::from("westlakes"),
                market: Market {
                    id: String::from("0200"),
                    name: String::from("San Antonio, TX"),
                    slug: String::from("san-antonio"),
                },
            },
            Cinema {
                id: String::from("0202"),
                name: String::from("Park North"),
                slug: String::from("park-north"),
                market: Market {
                    id: String::from("0200"),
                    name: String::from("San Antonio, TX"),
                    slug: String::from("san-antonio"),
                },
            },
            Cinema {
                id: String::from("0203"),
                name: String::from("Stone Oak"),
                slug: String::from("stone-oak"),
                market: Market {
                    id: String::from("0200"),
                    name: String::from("San Antonio, TX"),
                    slug: String::from("san-antonio"),
                },
            },
            Cinema {
                id: String::from("0301"),
                name: String::from("Winchester"),
                slug: String::from("winchester"),
                market: Market {
                    id: String::from("0300"),
                    name: String::from("Winchester, VA"),
                    slug: String::from("winchester"),
                },
            },
            Cinema {
                id: String::from("0401"),
                name: String::from("Littleton"),
                slug: String::from("littleton"),
                market: Market {
                    id: String::from("0400"),
                    name: String::from("Denver, CO"),
                    slug: String::from("denver"),
                },
            },
            Cinema {
                id: String::from("0402"),
                name: String::from("Sloans Lake"),
                slug: String::from("sloans-lake"),
                market: Market {
                    id: String::from("0400"),
                    name: String::from("Denver, CO"),
                    slug: String::from("denver"),
                },
            },
            Cinema {
                id: String::from("0501"),
                name: String::from("Loudoun"),
                slug: String::from("one-loudoun"),
                market: Market {
                    id: String::from("0500"),
                    name: String::from("Northern Virginia"),
                    slug: String::from("northern-virginia"),
                },
            },
            Cinema {
                id: String::from("0502"),
                name: String::from("Woodbridge"),
                slug: String::from("woodbridge"),
                market: Market {
                    id: String::from("0500"),
                    name: String::from("Northern Virginia"),
                    slug: String::from("northern-virginia"),
                },
            },
            Cinema {
                id: String::from("0601"),
                name: String::from("Mainstreet"),
                slug: String::from("mainstreet"),
                market: Market {
                    id: String::from("0600"),
                    name: String::from("Kansas City, MO"),
                    slug: String::from("kansas-city"),
                },
            },
            Cinema {
                id: String::from("0701"),
                name: String::from("Richardson"),
                slug: String::from("richardson"),
                market: Market {
                    id: String::from("0700"),
                    name: String::from("DFW Area, TX"),
                    slug: String::from("dfw"),
                },
            },
            Cinema {
                id: String::from("0702"),
                name: String::from("Cedars"),
                slug: String::from("cedars"),
                market: Market {
                    id: String::from("0700"),
                    name: String::from("DFW Area, TX"),
                    slug: String::from("dfw"),
                },
            },
            Cinema {
                id: String::from("0703"),
                name: String::from("Las Colinas"),
                slug: String::from("las-colinas"),
                market: Market {
                    id: String::from("0700"),
                    name: String::from("DFW Area, TX"),
                    slug: String::from("dfw"),
                },
            },
            Cinema {
                id: String::from("0706"),
                name: String::from("Lake Highlands"),
                slug: String::from("lake-highlands"),
                market: Market {
                    id: String::from("0700"),
                    name: String::from("DFW Area, TX"),
                    slug: String::from("dfw"),
                },
            },
            Cinema {
                id: String::from("0707"),
                name: String::from("Denton"),
                slug: String::from("denton"),
                market: Market {
                    id: String::from("0700"),
                    name: String::from("DFW Area, TX"),
                    slug: String::from("dfw"),
                },
            },
            Cinema {
                id: String::from("0801"),
                name: String::from("San Francisco"),
                slug: String::from("new-mission"),
                market: Market {
                    id: String::from("0800"),
                    name: String::from("San Francisco, CA"),
                    slug: String::from("sf"),
                },
            },
            Cinema {
                id: String::from("0901"),
                name: String::from("Yonkers"),
                slug: String::from("yonkers"),
                market: Market {
                    id: String::from("0900"),
                    name: String::from("Greater NY"),
                    slug: String::from("yonkers"),
                },
            },
            Cinema {
                id: String::from("1201"),
                name: String::from("Montecillo"),
                slug: String::from("montecillo"),
                market: Market {
                    id: String::from("1200"),
                    name: String::from("El Paso, TX"),
                    slug: String::from("el-paso"),
                },
            },
            Cinema {
                id: String::from("1301"),
                name: String::from("Marketplace"),
                slug: String::from("marketplace"),
                market: Market {
                    id: String::from("1300"),
                    name: String::from("New Braunfels, TX"),
                    slug: String::from("new-braunfels"),
                },
            },
            Cinema {
                id: String::from("1401"),
                name: String::from("Lubbock"),
                slug: String::from("lubbock"),
                market: Market {
                    id: String::from("1400"),
                    name: String::from("Lubbock, TX"),
                    slug: String::from("lubbock"),
                },
            },
            Cinema {
                id: String::from("1501"),
                name: String::from("Laredo"),
                slug: String::from("laredo"),
                market: Market {
                    id: String::from("1500"),
                    name: String::from("Laredo, TX"),
                    slug: String::from("laredo"),
                },
            },
            Cinema {
                id: String::from("1601"),
                name: String::from("Omaha"),
                slug: String::from("la-vista"),
                market: Market {
                    id: String::from("1600"),
                    name: String::from("Omaha, NE"),
                    slug: String::from("omaha"),
                },
            },
            Cinema {
                id: String::from("1901"),
                name: String::from("Chandler"),
                slug: String::from("chandler"),
                market: Market {
                    id: String::from("1900"),
                    name: String::from("Phoenix, AZ"),
                    slug: String::from("phoenix"),
                },
            },
            Cinema {
                id: String::from("1902"),
                name: String::from("Tempe"),
                slug: String::from("tempe"),
                market: Market {
                    id: String::from("1900"),
                    name: String::from("Phoenix, AZ"),
                    slug: String::from("phoenix"),
                },
            },
            Cinema {
                id: String::from("2001"),
                name: String::from("Corpus Christi"),
                slug: String::from("corpus-christi"),
                market: Market {
                    id: String::from("2000"),
                    name: String::from("Corpus Christi, TX"),
                    slug: String::from("corpus-christi"),
                },
            },
            Cinema {
                id: String::from("2101"),
                name: String::from("Downtown Brooklyn"),
                slug: String::from("downtown-brooklyn"),
                market: Market {
                    id: String::from("2100"),
                    name: String::from("Brooklyn"),
                    slug: String::from("nyc"),
                },
            },
       ]
    }

    pub fn to_cinema_id(cinema_id: &str) -> Option<String> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^\d+$") // only digits
                    .unwrap();
        }

        // if it's a match, then return it back
        if RE.is_match(cinema_id) {
            return Some(String::from(cinema_id));
        }

        // otherwise we need to look the thing up
        let cinemas = Cinema::list();

        let cinema_id =
            cinemas.iter()
                .find(|c| c.slug == cinema_id);
        
        match cinema_id {
            Some(c) => Some(c.id.clone()),
            None => None,
        }
    }

    pub fn from_calendar_file(path: &PathBuf) -> Result<(Cinema, Vec<Film>), Box<dyn Error>> {
        let contents = fs::read_to_string(path.to_str().unwrap())?;
        Cinema::from_calendar_data(&contents)
    }

    pub fn from_calendar_data(data: &str) -> Result<(Cinema, Vec<Film>), Box<dyn Error>> {
        let v: serde_json::Value = serde_json::from_str(data)?;

        match &v["Calendar"]["Cinemas"][0] {
            serde_json::Value::Null => Err(Box::new(InvalidCinemaData)),
            data => Cinema::from_calendar_json(data),
        }
    }

    pub fn from_calendar_json(json: &serde_json::Value) -> Result<(Cinema, Vec<Film>), Box<dyn Error>> {
        let name = cinema_name_from(json).unwrap();
        let id = cinema_id_from(json).unwrap();
        let slug = cinema_slug_from(json).unwrap();

        let market = Market::from_calendar_json(json)?;

        Ok((Cinema {
            id: id.to_string(),
            name: name.to_string(),
            slug: slug.to_string(),
            market,
            },
            Film::films_from_calendar(json)?,
        ))
    }

    pub fn get_calendar_data(cinema_id: &str) -> Result<String, Box<Error>>  {
        let url: &str = &format!("https://feeds.drafthouse.com/adcService/showtimes.svc/calendar/{}/", cinema_id);

        let body = reqwest::get(url)?
            .text()?;

        Ok(body)
    }

    pub fn sync_file(cinema_id: &str) -> Result<(Cinema, Vec<Film>), Box<dyn Error>> {
        let body = Cinema::get_calendar_data(cinema_id)?;

        // validate:
        let result = Cinema::from_calendar_data(&body)?;

        db::write_calendar_file(cinema_id, &body)?;

        Ok(result)
    }
}

fn cinema_name_from(data: &serde_json::Value) -> Option<&str> {
    data["CinemaName"].as_str()
}

fn cinema_id_from(data: &serde_json::Value) -> Option<&str> {
    data["CinemaId"].as_str()
}

fn cinema_slug_from(data: &serde_json::Value) -> Option<&str> {
    data["CinemaSlug"].as_str()
}

