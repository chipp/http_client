use super::{Error, HttpClient, Request, Response};
use serde::de::DeserializeOwned;
use serde_json;

impl<'a> HttpClient<'a> {
    pub async fn get<R, P>(&self, path: P) -> Result<R, Error>
    where
        R: DeserializeOwned + Send + 'static,
        P: IntoIterator,
        P::Item: AsRef<str>,
    {
        self.perform_request(self.new_request(path), parse_json)
            .await
    }

    pub async fn get_with_params<R, P, I, K, V>(&self, path: P, params: I) -> Result<R, Error>
    where
        R: DeserializeOwned + Send + 'static,
        P: IntoIterator,
        P::Item: AsRef<str>,
        I: IntoIterator,
        I::Item: std::borrow::Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.perform_request(self.new_request_with_params(path, params), parse_json)
            .await
    }
}

pub fn parse_json<T: DeserializeOwned>(req: Request, res: Response) -> Result<T, Error> {
    if res.status_code >= 200 && res.status_code < 300 {
        serde_json::from_slice(&res.body)
            .map_err(|err| Error::from(err))
            .into()
    } else {
        Err(Error::HttpError(req, res))
    }
}
