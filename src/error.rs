use crate::{Request, Response};

pub struct Error {
    pub request: Request,
    pub kind: ErrorKind,
}

pub enum ErrorKind {
    HttpError(Response),
    CurlError(curl::Error),
    JsonParseError(serde_json::Error),
}

impl From<(Request, curl::Error)> for Error {
    fn from(pair: (Request, curl::Error)) -> Error {
        Error {
            request: pair.0,
            kind: ErrorKind::CurlError(pair.1),
        }
    }
}

impl From<(Request, serde_json::Error)> for Error {
    fn from(pair: (Request, serde_json::Error)) -> Error {
        Error {
            request: pair.0,
            kind: ErrorKind::JsonParseError(pair.1),
        }
    }
}

impl From<(Request, Response)> for Error {
    fn from(pair: (Request, Response)) -> Error {
        Error {
            request: pair.0,
            kind: ErrorKind::HttpError(pair.1),
        }
    }
}

use std::fmt;

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;

        match &self.kind {
            CurlError(err) => f
                .debug_struct("CurlError")
                .field("request", &self.request)
                .field("error", &err)
                .finish(),
            JsonParseError(err) => f
                .debug_struct("JsonParseError")
                .field("request", &self.request)
                .field("error", &err)
                .finish(),
            HttpError(response) => f
                .debug_struct("HttpError")
                .field("request", &self.request)
                .field("response", &response)
                .finish(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;

        match &self.kind {
            CurlError(err) => curl::Error::fmt(&err, f),
            JsonParseError(err) => serde_json::Error::fmt(&err, f),
            HttpError(res) => write!(f, "HTTP Error: {}", res),
        }
    }
}
