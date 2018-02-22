use std::result;
use std::fmt;
use std::error;

// Parser/parsing related errors.
#[derive(Debug, Clone)]
pub struct Error {
    pub message: String
}

// ParserResult type.
pub type Result<T> = result::Result<T, Error>;

// Trait that defines anything that parseable.
pub trait Parseable {
    type ParsedData;
    fn parse(&self) -> Result<Self::ParsedData>;
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod parser_test {

    use parser;
    use std::error::Error;

    fn return_parsererror() -> parser::Result<String> {
        return Err(parser::Error {
            message: "failed to parse something".to_owned()
        });
    }

    #[test]
    fn test_parsererror_creation() {
        let result = return_parsererror();
        assert_eq!("failed to parse something".to_owned(), result.err().unwrap().description());
    }
}