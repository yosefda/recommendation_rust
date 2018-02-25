/// Common trait that describe all parseable data source.

use error;

pub trait Parseable {
    type Output;
    fn parse(&self) -> Result<Vec<Self::Output>, error::Error>;
}