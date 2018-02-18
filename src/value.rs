#[derive(Debug)]
pub struct MovieShowing {
    pub name: String,
    pub rating: u8,
    pub genres: Vec<String>,
    pub showings: Vec<String>
}


#[cfg(test)]
mod value_test {

    #[test]
    fn test_construction() {
        use value::MovieShowing;
        let movie = MovieShowing {
            name: "Movie".to_owned(),
            rating: 80,
            genres: vec!["Drama".to_owned()],
            showings: vec!["19:00:00+11:00".to_owned()]
        };

        assert_eq!(movie.name, "Movie".to_owned());
        assert_eq!(movie.rating, 80);
        assert_eq!(movie.genres, vec!["Drama"]);
        assert_eq!(movie.showings, vec!["19:00:00+11:00".to_owned()]);
    }
}