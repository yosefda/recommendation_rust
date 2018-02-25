/// ACME Movie JSON data.

use parser;
use value;
use serde_json;
use error;
use std::error::Error;


#[derive(Debug)]
pub struct JsonSource {
    pub json_string: String
}

impl JsonSource {
    pub fn new(json_string: String) -> Self {
            Self {
                json_string
            }
    }
}

impl parser::Parseable for JsonSource {
    type Output = value::MovieShowing;
    fn parse(&self) -> parser::Result<Vec<Self::Output>> {
        // deserialise json
        let json_data: serde_json::Value = try!(serde_json::from_str(&self.json_string));

        // iterate through json and build output
        let mut movie_showings = Vec::new();
        for entries in json_data.as_array().iter() {
            for entry in entries.iter() {
                movie_showings.push(
                    value::MovieShowing {
                        name: entry.get("name").unwrap().as_str().unwrap().to_owned(),
                        rating: entry.get("rating").unwrap().as_u64().unwrap() as u8,
                        genres: entry.get("genres").unwrap().as_array().unwrap().into_iter().map(|x| x.as_str().unwrap().to_owned()).collect::<Vec<String>>(),
                        showings: entry.get("showings").unwrap().as_array().unwrap().into_iter().map(|x| x.as_str().unwrap().to_owned()).collect::<Vec<String>>()
                    }
                )
            }
        }

        Ok(movie_showings)
    }
}


#[cfg(test)]
mod json_data_test {
    use acme_movie;
    use parser::Parseable;
    use value::MovieShowing;
    use error;
    use std::error::Error;

    #[test]
    fn test_new() {
        let source = acme_movie::JsonSource::new("[{\"key\": \"value\"}]".to_owned());
        assert_eq!("[{\"key\": \"value\"}]".to_owned(), source.json_string);
    }

    #[test]
    fn test_parse_empty_json() {
        let source = acme_movie::JsonSource::new("".to_owned());
        let result = source.parse();
        assert!(result.is_err());
        match result.err().unwrap() {
            error::Error::JsonParserError(_) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_parse_valid_json() {
        let json_source = r#"
                [
                    {
                        "name": "Moonlight",
                        "rating": 98,
                        "genres": [
                        "Drama"
                        ],
                        "showings": [
                        "18:30:00+11:00",
                        "20:30:00+11:00"
                        ]
                    },
                    {
                        "name": "Zootopia",
                        "rating": 92,
                        "genres": [
                            "Action & Adventure",
                            "Animation",
                            "Comedy"
                        ],
                        "showings": [
                            "19:00:00+11:00",
                            "21:00:00+11:00"
                        ]
                    }
                ]"#.to_owned();

        let source = acme_movie::JsonSource::new(json_source);
        let parsed_data = source.parse().unwrap();
        assert_eq!(2, parsed_data.len());

        assert_eq!("Moonlight".to_owned(), parsed_data[0].name);
        assert_eq!(98, parsed_data[0].rating);
        assert_eq!(vec!["Drama"], parsed_data[0].genres);
        assert_eq!(vec!["18:30:00+11:00", "20:30:00+11:00"], parsed_data[0].showings);
    }

    #[test]
    fn test_parse_invalid_json_data() {
        let source = acme_movie::JsonSource::new(
            r#"
                [
                    {
                        "name": "Moonlight",
                        "rating": 98,
                        "genres": [
                        "Drama"
                        ],
                        "showings": [
                        "18:30:00+11:00",
                        "20:30:00+11:00"
                        ]
                    },
                ]"#.to_owned()
        );

        let result = source.parse();
        assert!(result.is_err());
        match result.err().unwrap() {
            error::Error::JsonParserError(_) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_parse_empty_list_acme_json_data() {
        let source = acme_movie::JsonSource::new("[]".to_owned());
        assert!(source.parse().unwrap().is_empty());
    }
}