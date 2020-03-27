use crate::Response;

#[derive(Debug)]
pub enum Error {
    HttpError(Response),
    CurlError(curl::Error),
    JsonParseError(serde_json::Error),
}

impl From<curl::Error> for Error {
    fn from(error: curl::Error) -> Error {
        Error::CurlError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::JsonParseError(error)
    }
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            CurlError(err) => write!(f, "{}", err),
            JsonParseError(err) => write!(f, "{}", err),
            HttpError(err) => {
                if let Ok(body) = String::from_utf8(err.body.clone()) {
                    write!(f, "HTTP Error: {}\n{}", err.status_code, body)
                } else {
                    write!(f, "HTTP Error: {}", err.status_code)
                }
            }
        }
    }
}
