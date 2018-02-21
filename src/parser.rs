use std::result;
use std::fmt;
use std::error;

/// ParserResult type.
pub type ParserResult<T> = result::Result<T, ParserError>;

/// Trait that defines anything that parseable.
pub trait Parseable {
    fn parse(&self) -> ParserResult<()>;
}

/// Parser/parsing related errors.
#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ParserError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod parser_test {

    use std::error::Error;
    use parser::ParserError;

    fn return_parsererror() -> Result<String, ParserError> {
        return Err(ParserError {
            message: "failed to parse something".to_owned()
        });
    }

    #[test]
    fn test_parsererror_creation() {
        let result = return_parsererror();
        assert_eq!("failed to parse something".to_owned(), result.err().unwrap().description());
    }
}