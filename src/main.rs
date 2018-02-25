extern crate serde_json;

mod parser;
mod acme_movie;
mod value;
mod error;

use parser::Parseable;


fn main() {
    let data_source1 = acme_movie::JsonSource::new("".to_owned());
    println!("{:?}", data_source1.parse());

    let data_source2 = acme_movie::JsonSource::new("{}".to_owned());
    println!("{:?}", data_source2.parse());
}