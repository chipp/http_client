use std::str;
use std::thread;

use futures_channel::oneshot;

use ::curl::easy::{Easy, Form, List};
use url::{ParseError, Url};

mod request;
pub use request::{HttpMethod, Request};

mod response;
pub use response::Response;

pub mod curl {
    pub use ::curl::*;
}

pub mod json;

mod error;
pub use error::{Error, ErrorKind};

pub struct HttpClient<'a> {
    base_url: Url,
    interceptor: Option<Box<dyn Fn(&mut Easy) + Send + Sync + 'a>>,
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
        F: Fn(&mut Easy) + Send + Sync + 'a,
    {
        self.interceptor = Some(Box::new(interceptor))
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
        P: Fn(Request, Response) -> Result<R, Error> + Send + 'static,
    {
        let (tx, rx) = oneshot::channel::<Result<R, Error>>();
        let mut easy = Easy::new();
        easy.url(request.url.as_str()).unwrap();

        match request.method {
            HttpMethod::Get => (),
            HttpMethod::Post => easy.post(true).unwrap(),
        }

        if let Some(form_params) = &request.form {
            let mut form = Form::new();

            for (k, v) in form_params {
                let mut part = form.part(&k);
                part.contents(v.as_bytes());
                part.add().unwrap();
            }

            easy.httppost(form).unwrap();
        }

        if let Some(body) = &request.body {
            easy.post_field_size(body.len() as u64).unwrap();
            easy.post_fields_copy(&body).unwrap();
        }

        if let Some(headers) = &request.headers {
            let mut list = List::new();

            for header in headers {
                list.append(&header).unwrap();
            }

            easy.http_headers(list).unwrap();
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

            let mut attempts = 0;
            let mut transfer_error = None;

            loop {
                attempts += 1;

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

                match transfer.perform() {
                    Ok(()) => break,
                    Err(err) => {
                        if request.retry_count.is_none() || request.retry_count == Some(attempts) {
                            transfer_error = Some(err);
                            break;
                        }
                    }
                }
            }

            if let Some(err) = transfer_error {
                let _ = tx.send(Err((request, err).into()));
            } else {
                let status_code = easy.response_code().unwrap();

                let _ = tx.send(parse(
                    request,
                    Response {
                        status_code,
                        body,
                        headers,
                    },
                ));
            }
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

pub fn parse_void(req: Request, res: Response) -> Result<(), Error> {
    if res.status_code >= 200 && res.status_code < 300 {
        Ok(())
    } else {
        Err((req, res).into())
    }
}
