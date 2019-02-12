use serde_json;
use std::fs;
use std::error::Error;

pub struct Cinema {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub market: Market,
}

pub struct Market {
    pub id: String,
    pub name: String,
    pub slug: String,
}

impl Cinema {
    pub fn from_calendar_file(path: &str) -> Result<Cinema, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;

        let v: serde_json::Value = serde_json::from_str(&contents)?;

        let data = &v["Calendar"]["Cinemas"][0];

        Cinema::from_calendar_json(data)
    }

    pub fn from_calendar_json(json: &serde_json::Value) -> Result<Cinema, Box<dyn Error>> {
        let name = cinema_name_from(json).unwrap();
        let id = cinema_id_from(json).unwrap();
        let slug = cinema_slug_from(json).unwrap();

        let market_name = market_name_from(json).unwrap();
        let market_id = market_id_from(json).unwrap();
        let market_slug = market_slug_from(json).unwrap();

        Ok(Cinema {
            id: id.to_string(),
            name: name.to_string(),
            slug: slug.to_string(),
            market: Market {
                id: market_id.to_string(),
                name: market_name.to_string(),
                slug: market_slug.to_string(),
            }
        })
    }

    // pub fn name(&self) -> String {
        // let field = self.data["CinemaName"];

        // String::from(
            // field
                // .as_str()
                // .unwrap()
                // .to_string()
        // )
    // }
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

fn market_name_from(data: &serde_json::Value) -> Option<&str> {
    data["MarketName"].as_str()
}

fn market_id_from(data: &serde_json::Value) -> Option<&str> {
    data["MarketId"].as_str()
}

fn market_slug_from(data: &serde_json::Value) -> Option<&str> {
    data["MarketSlug"].as_str()
}
