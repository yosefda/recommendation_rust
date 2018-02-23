

use parser;
use value;
use serde_json;
use serde_json::Value;
use std::error::Error;
use std::iter::Iterator;

// Struct that contains JSON data from ACME.
pub struct ACMEJsonData {
    pub json: String
}


impl parser::Parseable for ACMEJsonData {
    type ParsedData = Vec<value::MovieShowing>;
    fn parse(&self) -> parser::Result<Self::ParsedData> {
        let json_value: Value  = match serde_json::from_str(&self.json) {
            Ok(success_value) => success_value,
            Err(err) => return Err(parser::Error{
                message: err.description().to_owned(),
            })
        };

        let mut movie_showings = Vec::new();
        for entries in json_value.as_array().iter() {
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
mod acme_json_data_test {

    use acme_json_data::ACMEJsonData;
    use parser::Parseable;

    #[test]
    fn test_acme_json_data_creation() {
        let data = ACMEJsonData {
            json: r#"
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
                    }
                ]"#.to_owned()
        };

        assert_eq!(r#"
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
                    }
                ]"#.to_owned()  , data.json);
    }

    #[test]
    fn test_parse_acme_json_data() {
        let data = ACMEJsonData {
            json: r#"
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
                ]"#.to_owned()
        };

        let parsed_data = data.parse().unwrap();
        assert_eq!(2, parsed_data.len());

        assert_eq!("Moonlight".to_owned(), parsed_data[0].name);
        assert_eq!(98, parsed_data[0].rating);
        assert_eq!(vec!["Drama"], parsed_data[0].genres);
        assert_eq!(vec!["18:30:00+11:00", "20:30:00+11:00"], parsed_data[0].showings);
    }

    #[test]
    fn test_parse_empty_acme_json_data() {
        let data = ACMEJsonData{
            json: "".to_owned()
        };

        assert!(data.parse().is_err());
    }

    #[test]
    fn test_parse_invalid_acme_json_data() {
        let data = ACMEJsonData {
            json: r#"
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
        };

        assert!(data.parse().is_err());
    }

    #[test]
    fn test_parse_empty_list_acme_json_data() {
        let data = ACMEJsonData {
            json: r#"
                []"#.to_owned()
        };

        assert!(data.parse().unwrap().is_empty());
    }
}
