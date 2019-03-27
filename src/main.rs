use alamo_movies;
use alamo_movies::Cinema;

fn main() {
    // first, read the file into a string
    let path = "0002.calendar.json";

    let cinema = Cinema::from_calendar_file(&path).expect("cannot load file");
    let name = cinema.name;

    // let data = get_data(&path).expect("Whoops!");

    // let v: serde_json::Value = serde_json::from_str(&data).expect("can't parse");

    // let cinema = &v["Calendar"]["Cinemas"][0];
    // let jsonname = &cinema["CinemaName"];
    // let name = jsonname.as_str().expect("Cannot find");

    println!("read: {}", name);
}

// fn get_data(path: &str) -> Result<String, Box<dyn Error>> {
    // let contents = fs::read_to_string(path)?;

    // Ok(contents)
// }

