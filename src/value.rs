// Value object for movie showing.
#[derive(Debug)]
pub struct MovieShowing {
    pub name: String,
    pub rating: u8,
    pub genres: Vec<String>,
    pub showings: Vec<String>
}

#[cfg(test)]
mod value_test {

    use value::MovieShowing;

    #[test]
    fn test_movieshowing_creation() {
        let movie = MovieShowing {
            name: "Movie".to_owned(),
            rating: 80,
            genres: vec!["Drama".to_owned()],
            showings: vec!["19:00:00+11:00".to_owned()]
        };

        assert_eq!("Movie".to_owned(), movie.name);
        assert_eq!(80, movie.rating);
        assert_eq!(vec!["Drama"], movie.genres);
        assert_eq!(vec!["19:00:00+11:00".to_owned()], movie.showings);
    }
}