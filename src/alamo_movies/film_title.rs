use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bare_title() {
        if let Some(film_title) = FilmTitle::parse("FREE SOLO") {
            assert_eq!(film_title.show_type, "");
            assert_eq!(film_title.title , "FREE SOLO");
            assert_eq!(film_title.suffix, "");
        } else {
            panic!("Fail");
        }
    }

    #[test]
    fn parse_title_with_year() {
        if let Some(film_title) = FilmTitle::parse("US (2009)") {
            assert_eq!(film_title.show_type, "");
            assert_eq!(film_title.title , "US (2009)");
            assert_eq!(film_title.suffix, "");
        } else {
            panic!("Fail!");
        }
    }

    #[test]
    fn parses_movie_with_type() {
        if let Some(film_title) = FilmTitle::parse("Weird Wednesday: SUPERMAN III") {
            assert_eq!(film_title.show_type, "Weird Wednesday");
            assert_eq!(film_title.title , "SUPERMAN III");
            assert_eq!(film_title.suffix, "");
        } else {
            panic!("fail");
        }
    }

    #[test]
    fn parses_movie_with_type_and_colon() {
        if let Some(film_title) = FilmTitle::parse("Terror Tuesday: FRIDAY THE 13TH: THE FINAL CHAPTER") {
            assert_eq!(film_title.show_type, "Terror Tuesday");
            assert_eq!(film_title.title, "FRIDAY THE 13TH: THE FINAL CHAPTER");
            assert_eq!(film_title.suffix, "");
        } else {
            panic!("fail");
        }
    }
    
    #[test]
    fn parses_movie_with_type_and_symbols() {
        if let Some(film_title) = FilmTitle::parse("Weird Wednesday: KNIFE + HEART in 35mm") {
            assert_eq!(film_title.show_type, "Weird Wednesday");
            assert_eq!(film_title.title, "KNIFE + HEART");
            assert_eq!(film_title.suffix, "in 35mm");
        } else {
            panic!("fail");
        }
    }

    #[test]
    fn parses_movie_with_type_and_symbols_and_numbers() {
        if let Some(film_title) = FilmTitle::parse("Terror Tuesday: FINAL DESTINATION 2") {
            assert_eq!(film_title.show_type, "Terror Tuesday");
            assert_eq!(film_title.title, "FINAL DESTINATION 2");
            assert_eq!(film_title.suffix, "");
        } else {
            panic!("fail");
        }
    }

}

pub struct FilmTitle {
    pub title: String,
    pub show_type: String,
    pub suffix: String,
}

impl FilmTitle {
    pub fn parse(title: &str) -> Option<FilmTitle> {
        lazy_static! {
             static ref RE: Regex =
                 Regex::new(r"^(([^:]+[a-z]+):\s+)?([^a-z]+)(\s(.*[a-z].*))?$")
                     .unwrap();
        }

        // eprintln!("[DEBUG] Parsing: {}", name);

        match RE.captures(title) {
            None => Some(FilmTitle {
                title: String::from(title),
                show_type: String::from(""),
                suffix: String::from(""),
            }), // no match, so return nothing.
            Some(cap) => {
                //eprintln!("captures: {:?}", cap);
                // 2 - type
                // 3 - name
                // 5 - suffix

                let show_type = String::from(cap.get(2).map_or("", |c| c.as_str()));
                let title = String::from(cap.get(3).map_or("", |c| c.as_str()));
                let suffix = String::from(cap.get(5).map_or("", |c| c.as_str()));

                Some(FilmTitle {
                    title,
                    show_type,
                    suffix,
                })
            }
        }
        
        
    }
}
