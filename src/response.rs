pub struct Response {
    pub status_code: u32,
    pub body: Vec<u8>,
    pub headers: Vec<String>,
}

use std::fmt;

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Response");

        if let Ok(body) = String::from_utf8(self.body.clone()) {
            debug.field("body", &body);
        }

        debug
            .field("status_code", &self.status_code)
            .field("headers", &self.headers)
            .finish()
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Status: {}", self.status_code)?;

        if let Ok(body) = std::str::from_utf8(&self.body) {
            write!(f, "Body: {}", body)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let res = Response {
            status_code: 404,
            body: Vec::from("Not Found!!!".as_bytes()),
            headers: vec!["X-Custom: None".to_string()],
        };

        assert_eq!(
            format!("{:?}", res),
            r#"Response { body: "Not Found!!!", status_code: 404, headers: ["X-Custom: None"] }"#
        );
    }

    #[test]
    fn test_display() {
        let res = Response {
            status_code: 404,
            body: Vec::from("Not Found!!!".as_bytes()),
            headers: vec!["X-Custom: None".to_string()],
        };

        assert_eq!(format!("{}", res), "Status: 404\nBody: Not Found!!!");
    }
}
