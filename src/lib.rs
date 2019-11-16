use curl::easy::Easy;
use serde::de::DeserializeOwned;
use serde_json;
use std::borrow::Borrow;
use url::{ParseError, Url};

#[derive(Debug)]
pub enum Error {
    HttpError(u32),
    CurlError(curl::Error),
    ParseError(serde_json::Error),
}

impl From<curl::Error> for Error {
    fn from(error: curl::Error) -> Error {
        Error::CurlError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::ParseError(error)
    }
}

pub struct HttpClient {
    base_url: Url,
}

impl HttpClient {
    // Public API

    pub fn new(base_url: &str) -> Result<HttpClient, ParseError> {
        let base_url = Url::parse(base_url)?;
        Ok(HttpClient { base_url })
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        self.do_get(self.prepare_url_with_path(path))
    }

    pub fn get_with_params<T, I, K, V>(&self, path: &str, iter: I) -> Result<T, Error>
    where
        T: DeserializeOwned,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut url = self.prepare_url_with_path(path);
        url.query_pairs_mut().extend_pairs(iter);
        self.do_get(url)
    }

    // Private API

    fn prepare_url_with_path(&self, path: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path(path);
        url
    }

    fn do_get<T: DeserializeOwned>(&self, url: Url) -> Result<T, Error> {
        let mut response = Vec::new();
        let mut easy = Easy::new();
        easy.url(url.as_str()).unwrap();

        {
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                    response.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        let code = easy.response_code().unwrap();

        if code >= 200 && code < 300 {
            let response: T = serde_json::from_slice(&response)
                .map_err(|err| Error::from(err))
                .unwrap();

            Ok(response)
        } else {
            eprintln!("{}", String::from_utf8_lossy(&response));
            Err(Error::HttpError(code))
        }
    }
}
