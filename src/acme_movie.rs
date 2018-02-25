/// ACME Movie JSON data.

use parser;
use value;

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
    fn parse(&self) -> Result<Vec<Self::Output>, &str> {
        // TODO: implement this
        println!("{:?}", self.json_string);
        Ok(vec![value::MovieShowing::new("something".to_owned(), 3, vec!["Drama".to_owned()], vec!["18:30:00+11:00".to_owned()])])
    }
}


#[cfg(test)]
mod json_data_test {

    use acme_movie;
    use parser::Parseable;
    use value::MovieShowing;

    #[test]
    fn test_new() {
        let source = acme_movie::JsonSource::new("[{\"key\": \"value\"}]".to_owned());
        assert_eq!("[{\"key\": \"value\"}]".to_owned(), source.json_string);
    }

    #[test]
    fn test_parse() {
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
        println!("{:?}", source.parse());
    }
}