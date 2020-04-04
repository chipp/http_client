use crate::{Request, Response};

pub enum Error {
    HttpError(Request, Response),
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

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            CurlError(err) => curl::Error::fmt(err, f),
            JsonParseError(err) => serde_json::Error::fmt(err, f),
            HttpError(req, res) => f
                .debug_struct("HttpError")
                .field("request", &req)
                .field("response", &res)
                .finish(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            CurlError(err) => curl::Error::fmt(err, f),
            JsonParseError(err) => serde_json::Error::fmt(err, f),
            HttpError(_, res) => write!(f, "HTTP Error: {}", res),
        }
    }
}
