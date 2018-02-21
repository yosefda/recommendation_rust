/// A trait for feed data source that is parseable into value object for further usage/processing

use error::ParserError;
use std::result;

pub type ParserResult<T> = result::Result<T, ParserError>;

pub trait Parseable {
    fn parse(&self) -> ParserResult<()>;
}