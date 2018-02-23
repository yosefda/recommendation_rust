use std::result;
use std::fmt;
use std::error;

// Fetcher/fetching related errors.
#[derive(Debug, Clone)]
pub struct Error {
    pub message: String
}

// FetcherResult type.
pub type Result<T> = result::Result<T, Error>;

// Trait that defines a fetcher.
pub trait Fetcher {
    fn fetch(&self) -> Result<String>;
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
mod fetcher_test {

    use fetcher::fetcher;
    use std::error::Error;

    fn return_fetcherrerror() -> fetcher::Result<String> {
        return Err(fetcher::Error {
            message: "failed to fetch something".to_owned()
        });
    }

    #[test]
    fn test_fetchererror_creation() {
        let result = return_fetcherrerror();
        assert_eq!("failed to fetch something".to_owned(), result.err().unwrap().description());
    }
}