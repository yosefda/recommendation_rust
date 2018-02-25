/// Trait that defines a fetcher.

use error;
use std::result as std_result;

pub type Result<T> = std_result::Result<T, error::Error>;

pub trait Fetcher {
    type Output;
    fn fetch(&self) -> Result<Self::Output>;
}


/// HTTP fetcher using reqwest
pub mod reqwest {

    use fetcher;
    use reqwest;
    use std::io::Read;

    #[derive(Debug)]
    pub struct HttpFetcher {
        pub uri: String
    }

    impl HttpFetcher {
        pub fn new(uri: String) -> Self {
            Self {
                uri
            }
        }
    }

    impl fetcher::Fetcher for HttpFetcher {
        type Output = String;

        fn fetch(&self) -> fetcher::Result<Self::Output> {
            let mut text  = try!(reqwest::get(&self.uri));

            let mut body = String::new();
            text.read_to_string(&mut body);

            Ok(body)
        }
    }
}


#[cfg(test)]
mod reqwest_http_fetcher_test {

    use fetcher::reqwest;
    use fetcher::Fetcher;
    use error;

    #[test]
    fn test_new() {
        let tfetcher = reqwest::HttpFetcher::new("https://somewhere.at/data".to_owned());
        assert_eq!("https://somewhere.at/data".to_owned(), tfetcher.uri);
    }

    #[test]
    fn test_failed_to_fetch() {
        let tfetcher = reqwest::HttpFetcher::new("https://somewhere.at/data".to_owned());
        let result = tfetcher.fetch();

        assert!(result.is_err());
        match result.err().unwrap() {
            error::Error::HttpFetcherError(_) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_success_to_fetch() {
        let tfetcher = reqwest::HttpFetcher::new("http://pastebin.com/raw/cVyp3McN".to_owned());
        let result = tfetcher.fetch();

        assert!(!result.unwrap().is_empty());
    }
}