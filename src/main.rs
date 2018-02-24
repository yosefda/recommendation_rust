extern crate serde_json;
extern crate reqwest;

mod value;
use value::*;
mod parser;
use std::error::Error;
mod acme_json_data;
mod fetcher;
mod strategy;

fn return_parsererror() -> parser::Result<String> {
    return Err(parser::Error {
        message: "failed to parse something".to_owned()
    });
}

fn main() {
    let moonlight = MovieShowing {
        name: "Moonlight".to_owned(),
        rating: 98,
        genres: vec!["Drama".to_owned()],
        showings: vec!["18:30:00+11:00".to_owned(), "20:30:00+11:00".to_owned()]
    };

    println!("{:?}", moonlight);

    let result = return_parsererror();
    println!("{:?}", result.err().unwrap().description());
}
