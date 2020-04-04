use std::borrow::Borrow;
use url::Url;

pub struct Request {
    pub url: Url,
    pub method: HttpMethod,
    pub headers: Option<Vec<String>>,
    pub form: Option<Vec<(String, String)>>,
    pub body: Option<Vec<u8>>,
}

use std::fmt;

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .field("url", &self.url)
            .finish()
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
}

impl Default for HttpMethod {
    fn default() -> HttpMethod {
        HttpMethod::Get
    }
}

impl Request {
    pub fn new(url: Url) -> Request {
        Request {
            url: url,
            method: HttpMethod::default(),
            form: None,
            headers: None,
            body: None,
        }
    }
}

impl Request {
    pub fn set_method(&mut self, method: HttpMethod) {
        self.method = method
    }

    pub fn add_header<H, V>(&mut self, header: H, value: V)
    where
        H: AsRef<str>,
        V: AsRef<str>,
    {
        if let None = self.headers {
            self.headers = Some(vec![]);
        }

        self.headers
            .as_mut()
            .unwrap()
            .push(format!("{}: {}", header.as_ref(), value.as_ref()));
    }

    pub fn set_form<I, K, V>(&mut self, form_iter: I)
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut form = vec![];

        for pair in form_iter.into_iter() {
            let &(ref k, ref v) = pair.borrow();
            form.push((String::from(k.as_ref()), String::from(v.as_ref())));
        }

        self.form = Some(form)
    }

    pub fn set_urlencoded_params<I, K, V>(&mut self, params: I)
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut serializer = url::form_urlencoded::Serializer::new(String::default());

        for pair in params.into_iter() {
            let &(ref k, ref v) = pair.borrow();
            serializer.append_pair(k.as_ref(), v.as_ref());
        }

        self.body = Some(serializer.finish().into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let mut req = Request::new(Url::parse("https://example.com").unwrap());
        req.set_method(HttpMethod::Post);

        assert_eq!(
            format!("{:?}", req),
            r#"Request { method: Post, url: "https://example.com/" }"#
        );
    }
}
