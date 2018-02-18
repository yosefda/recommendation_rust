mod value;
use value::*;

fn main() {
    let moonlight = MovieShowing {
        name: "Moonlight".to_owned(),
        rating: 98,
        genres: vec!["Drama".to_owned()],
        showings: vec!["18:30:00+11:00".to_owned(), "20:30:00+11:00".to_owned()]
    };

    println!("{:?}", moonlight);
}
