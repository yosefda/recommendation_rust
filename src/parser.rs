/// Common trait that describe all parseable data source.

use error;
use std::result as std_result;

pub type Result<T> = std_result::Result<T, error::Error>;

pub trait Parseable {
    type Output;
    fn parse(&self) -> Result<Vec<Self::Output>>;
}