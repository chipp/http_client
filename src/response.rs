use std::fmt;

use crate::hexdump::hexdump;

pub struct Response {
    pub status_code: u32,
    pub body: Vec<u8>,
    pub headers: Vec<String>,
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Response");

        debug
            .field("status_code", &self.status_code)
            .field("headers", &self.headers)
            .finish()?;

        if !self.body.is_empty() {
            writeln!(f)?;
            hexdump(&self.body, f)
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Status: {}", self.status_code)?;

        if !self.body.is_empty() {
            hexdump(&self.body, f)
        } else {
            Ok(())
        }
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
            r#"Response { status_code: 404, headers: ["X-Custom: None"] }
00000000  4e 6f 74 20 46 6f 75 6e  64 21 21 21              |Not.Found!!!|"#
        );
    }

    #[test]
    fn test_display() {
        let res = Response {
            status_code: 404,
            body: Vec::from("Not Found!!!".as_bytes()),
            headers: vec!["X-Custom: None".to_string()],
        };

        assert_eq!(
            format!("{}", res),
            r#"Status: 404
00000000  4e 6f 74 20 46 6f 75 6e  64 21 21 21              |Not.Found!!!|"#
        );
    }
}
