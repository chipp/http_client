use curl::easy::{Form, List};
use std::borrow::Borrow;
use url::Url;

pub struct Request {
    pub(crate) url: Url,
    pub(crate) method: HttpMethod,
    pub(crate) headers: Option<List>,
    pub(crate) form: Option<Form>,
    pub(crate) body: Option<Vec<u8>>,
}

#[derive(Debug)]
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
            self.headers = Some(List::new());
        }

        self.headers
            .as_mut()
            .unwrap()
            .append(&format!("{}: {}", header.as_ref(), value.as_ref()))
            .unwrap();
    }

    pub fn set_form<I, K, V>(&mut self, form_iter: I)
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut form = Form::new();

        for pair in form_iter.into_iter() {
            let &(ref k, ref v) = pair.borrow();

            let mut part = form.part(k.as_ref());
            part.contents(v.as_ref().as_bytes());
            part.add().unwrap();
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
