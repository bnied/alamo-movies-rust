use serde_json;
use std::fs;
use std::error::Error;

pub struct Cinema {
    pub name: String
    // data: serde_json::Value
}

impl Cinema {
    pub fn from_calendar_file(path: &str) -> Result<Cinema, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;

        let v: serde_json::Value = serde_json::from_str(&contents)?;

        let cinema_data = &v["Calendar"]["Cinemas"][0];

        let name_field = &cinema_data["CinemaName"].as_str().unwrap();

        Ok(Cinema {
            name: name_field.to_string()
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
