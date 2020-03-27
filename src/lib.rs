use std::str;
use std::thread;

use futures_channel::oneshot;

use ::curl::easy::Easy;
use url::{ParseError, Url};

mod request;
pub use request::{HttpMethod, Request};

pub mod curl {
    pub use ::curl::*;
}

mod json;

mod error;
pub use error::Error;

#[derive(Debug)]
pub struct Response {
    pub status_code: u32,
    pub body: Vec<u8>,
    pub headers: Vec<String>,
}

pub struct HttpClient<'a> {
    base_url: Url,
    interceptor: Option<Box<dyn Fn(&mut Easy) + 'a>>,
}

impl<'a> HttpClient<'a> {
    pub fn new<U>(base_url: U) -> Result<HttpClient<'a>, ParseError>
    where
        U: AsRef<str>,
    {
        let base_url = Url::parse(base_url.as_ref())?;
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
}

impl<'a> HttpClient<'a> {
    fn prepare_url_with_path<P>(&self, path: P) -> Url
    where
        P: IntoIterator,
        P::Item: AsRef<str>,
    {
        let mut url = self.base_url.clone();
        url.path_segments_mut().unwrap().pop_if_empty().extend(path);
        url
    }

    pub async fn perform_request<R: Send + 'static, P>(
        &self,
        request: Request,
        parse: P,
    ) -> Result<R, Error>
    where
        P: Fn(Response) -> Result<R, Error> + Send + 'static,
    {
        let (tx, rx) = oneshot::channel::<Result<R, Error>>();
        let mut easy = Easy::new();
        easy.url(request.url.as_str()).unwrap();

        match request.method {
            HttpMethod::Get => (),
            HttpMethod::Post => easy.post(true).unwrap(),
        }

        if let Some(form) = request.form {
            easy.httppost(form).unwrap();
        }

        if let Some(body) = request.body {
            easy.post_field_size(body.len() as u64).unwrap();
            easy.post_fields_copy(&body).unwrap();
        }

        if let Some(headers) = request.headers {
            easy.http_headers(headers).unwrap();
        }

        {
            match self.interceptor {
                Some(ref interceptor) => &interceptor(&mut easy),
                None => &(),
            };
        }

        thread::spawn(move || {
            let mut body = Vec::new();
            let mut headers = Vec::new();

            {
                let mut transfer = easy.transfer();
                transfer
                    .write_function(|data| {
                        body.extend_from_slice(data);
                        Ok(data.len())
                    })
                    .unwrap();

                transfer
                    .header_function(|header| {
                        headers.push(str::from_utf8(header).unwrap().trim_end().to_string());
                        true
                    })
                    .unwrap();

                transfer.perform().unwrap();
            }

            let status_code = easy.response_code().unwrap();

            let _ = tx.send(parse(Response {
                status_code,
                body,
                headers,
            }));
        });

        rx.await.unwrap()
    }
}

impl<'a> HttpClient<'a> {
    pub fn new_request<P>(&self, path: P) -> Request
    where
        P: IntoIterator,
        P::Item: AsRef<str>,
    {
        Request::new(self.prepare_url_with_path(path))
    }

    pub fn new_request_with_params<P, I, K, V>(&self, path: P, params: I) -> Request
    where
        P: IntoIterator,
        P::Item: AsRef<str>,
        I: IntoIterator,
        I::Item: std::borrow::Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut url = self.prepare_url_with_path(path);
        url.query_pairs_mut().extend_pairs(params);

        Request::new(url)
    }

    pub fn new_request_with_url(&self, url: Url) -> Request {
        Request::new(url)
    }
}
