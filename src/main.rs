mod alamo_movies;
use crate::alamo_movies::cinema::Cinema;

fn main() {
    // first, read the file into a string
    let path = "0002.calendar.json";

    let cinema = Cinema::from_calendar_file(&path).expect("cannot load file");

    // let data = get_data(&path).expect("Whoops!");

    // let v: serde_json::Value = serde_json::from_str(&data).expect("can't parse");

    // let cinema = &v["Calendar"]["Cinemas"][0];
    // let jsonname = &cinema["CinemaName"];
    // let name = jsonname.as_str().expect("Cannot find");

    println!("cinema: {} / {} - {}", cinema.name, cinema.id, cinema.slug);
    println!("market: {} / {} - {}", cinema.market.name, cinema.market.id, cinema.market.slug);
}

// fn get_data(path: &str) -> Result<String, Box<dyn Error>> {
    // let contents = fs::read_to_string(path)?;

    // Ok(contents)
// }

