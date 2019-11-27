use std::borrow::Borrow;
use std::thread;

use futures_channel::oneshot;

use serde::de::DeserializeOwned;
use serde_json;

use curl::easy::Easy;
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

pub struct HttpClient<'a> {
    base_url: Url,
    interceptor: Option<Box<dyn Fn(&mut Easy) + 'a>>,
}

impl<'a> HttpClient<'a> {
    pub fn new<'b>(base_url: &'b str) -> Result<HttpClient<'a>, ParseError> {
        let base_url = Url::parse(base_url)?;
        Ok(HttpClient {
            base_url,
            interceptor: None,
        })
    }

    pub fn set_interceptor<F>(&mut self, interceptor: F)
    where
        F: Fn(&mut Easy) + 'a,
    {
        self.interceptor = Some(Box::from(interceptor))
    }

    pub async fn get<T: DeserializeOwned + Send + 'static, U>(&self, path: U) -> Result<T, Error>
    where
        U: AsRef<str>,
    {
        self.do_get(self.prepare_url_with_path(path)).await
    }

    pub async fn get_with_params<T, U, I, K, V>(&self, path: U, iter: I) -> Result<T, Error>
    where
        T: DeserializeOwned + Send + 'static,
        U: AsRef<str>,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut url = self.prepare_url_with_path(path);
        url.query_pairs_mut().extend_pairs(iter);
        self.do_get(url).await
    }

    // Private API

    fn prepare_url_with_path<U>(&self, path: U) -> Url
    where
        U: AsRef<str>,
    {
        let mut url = self.base_url.clone();
        url.set_path(path.as_ref());
        url
    }

    async fn do_get<T: DeserializeOwned + Send + 'static>(&self, url: Url) -> Result<T, Error> {
        let (tx, rx) = oneshot::channel::<Result<T, Error>>();

        let mut response = Vec::new();
        let mut easy = Easy::new();
        easy.url(url.as_str()).unwrap();

        match self.interceptor {
            Some(ref interceptor) => &interceptor(&mut easy),
            None => &(),
        };

        thread::spawn(move || {
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

                let _ = tx.send(Ok(response));
            } else {
                eprintln!("{}", String::from_utf8_lossy(&response));
                let _ = tx.send(Err(Error::HttpError(code)));
            }
        });

        rx.await.unwrap()
    }
}
