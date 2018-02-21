/// Definitions for custom error types.

use std::fmt;
use std::error;

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
mod parsererror_test {

    use std::error::Error;
    use error::ParserError;

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