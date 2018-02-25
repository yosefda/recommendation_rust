/// Defines values used in this application.

#[derive(Debug)]
pub struct MovieShowing {
    pub name: String,
    pub rating: u8,
    pub genres: Vec<String>,
    pub showings: Vec<String>
}

impl MovieShowing {
    pub fn new(name: String, rating: u8, genres: Vec<String>, showings: Vec<String>) -> Self {
        Self {
            name: String::from(name),
            rating,
            genres,
            showings
        }
    }
}


#[cfg(test)]
mod test_movie_showing {

    use value;

    #[test]
    fn test_new() {
        let movie = value::MovieShowing::new(
            "Moonlight".to_owned(),
            98,
            vec!["Drama".to_owned()],
            vec!["18:30:00+11:00".to_owned(), "20:30:00+11:00".to_owned()]
        );

        assert_eq!("Moonlight".to_owned(), movie.name);
        assert_eq!(98, movie.rating);
        assert_eq!(vec!["Drama".to_owned()], movie.genres);
        assert_eq!(vec!["18:30:00+11:00".to_owned(), "20:30:00+11:00".to_owned()], movie.showings);
    }
}