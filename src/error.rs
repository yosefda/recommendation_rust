/// Defines all the errors in this application.

use serde_json;
use std::error as std_error;
use std::fmt;


#[derive(Debug)]
pub enum Error {
    JsonParserError(serde_json::Error)
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonParserError(err)
    }
}

impl std_error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::JsonParserError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&std_error::Error> {
        match *self {
            Error::JsonParserError(ref err) => Some(err)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::JsonParserError(ref err) => err.fmt(f)
        }
    }
}