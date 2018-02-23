use fetcher::fetcher;
use reqwest;
use std::error::Error;
use std::io::Read;


pub struct HyperHttpFetcher {
    uri: String
}

impl fetcher::Fetcher for HyperHttpFetcher {
    fn fetch(&self) -> fetcher::Result<String> {


        let mut text  = match reqwest::get(&self.uri) {
            Ok(success_value) => success_value,
            Err(err) => return Err(fetcher::Error{
                message: err.description().to_owned(),
            })
        };

        let mut body = String::new();
        text.read_to_string(&mut body);

        Ok(body)
    }
}

#[cfg(test)]
mod acme_json_data_test {
    use fetcher::reqwest_http_fetcher;
    use fetcher::fetcher;
    use fetcher::fetcher::Fetcher;

    #[test]
    fn test_fetch() {
        let http_fetcher = reqwest_http_fetcher::HyperHttpFetcher {
            uri: "https://pastebin.com/raw/cVyp3McN".to_owned()
        };

        assert!(!http_fetcher.fetch().unwrap().is_empty());
    }
}