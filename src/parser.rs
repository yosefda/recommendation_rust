/// Common trait that describe all parseable data source.

pub trait Parseable {
    type Output;
    fn parse(&self) -> Result<Vec<Self::Output>, &str>;
}