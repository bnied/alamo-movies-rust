use std::error::Error;

pub struct Movie {
    pub id: String,
    pub name: String,
    pub year: String,
    pub rating: String,
    pub runtime: String,
    pub slug: String
}

impl Movie {
    pub fn from_json(json: &serde_json::Value) -> Result<Movie, Box<dyn Error>> {
        let id = id_from(json).unwrap();
        let name = name_from(json).unwrap();
        let year = year_from(json).unwrap();
        let rating = rating_from(json).unwrap();
        let runtime = runtime_from(json).unwrap();
        let slug = slug_from(json).unwrap();

        Ok(Movie {
            id: id.to_string(),
            name: name.to_string(),
            year: year.to_string(),
            rating: rating.to_string(),
            runtime: runtime.to_string(),
            slug: slug.to_string(),
        })
    }
}


fn id_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmId"].as_str()
}

fn name_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmName"].as_str()
}

fn year_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmYear"].as_str()
}

fn rating_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmRating"].as_str()
}

fn runtime_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmRuntime"].as_str()
}

fn slug_from(data: &serde_json::Value) -> Option<&str> {
    data["FilmSlug"].as_str()
}

