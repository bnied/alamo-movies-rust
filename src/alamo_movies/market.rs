use serde_json;
use std::error::Error;

pub struct Market {
    pub id: String,
    pub name: String,
    pub slug: String,
}

impl Market {
    pub fn from_calendar_json(json: &serde_json::Value) -> Result<Market, Box<dyn Error>> {
        let name = market_name_from(json).unwrap();
        let id = market_id_from(json).unwrap();
        let slug = market_slug_from(json).unwrap();

        Ok(Market {
            id: id.to_string(),
            name: name.to_string(),
            slug: slug.to_string(),
        })
    }
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

