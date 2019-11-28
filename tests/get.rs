use curl::easy::Auth;
use futures::executor::block_on;
use http_client::HttpClient;
use serde_derive::Deserialize;

#[test]
fn test_get() {
    #[derive(Deserialize)]
    struct Response {
        url: String,
    }

    let url = url::Url::parse("https://httpbin.org/").unwrap();
    let http_client = HttpClient::new(url.as_ref()).unwrap();

    assert_eq!(
        block_on(http_client.get::<Response>("/get")).unwrap().url,
        "https://httpbin.org/get"
    );
}

#[test]
fn test_get_join() {
    #[derive(Deserialize)]
    struct Response {
        url: String,
    }

    let http_client = HttpClient::new("https://httpbin.org/").unwrap();
    let results = block_on(futures::future::join(
        http_client.get::<Response>("/delay/2"),
        http_client.get::<Response>("/delay/1"),
    ));

    assert_eq!(results.0.unwrap().url, "https://httpbin.org/delay/2");
    assert_eq!(results.1.unwrap().url, "https://httpbin.org/delay/1");
}

#[test]
fn test_get_with_params() {
    use std::collections::HashMap;

    #[derive(Deserialize)]
    struct Response {
        url: String,
        args: HashMap<String, String>,
    }

    let http_client = HttpClient::new("https://httpbin.org/").unwrap();

    let params = vec![("key1", "value1"), ("key2", "value2")];
    let response =
        block_on(http_client.get_with_params::<Response, _, _, _>("/get", params)).unwrap();

    assert_eq!(
        response.url,
        "https://httpbin.org/get?key1=value1&key2=value2"
    );
    assert_eq!(
        response.args,
        [
            ("key1".to_owned(), "value1".to_owned()),
            ("key2".to_owned(), "value2".to_owned())
        ]
        .iter()
        .cloned()
        .collect()
    )
}

#[test]
fn test_interceptor() {
    #[derive(Deserialize)]
    struct Response {
        authenticated: bool,
        user: String,
    }

    let mut http_client = HttpClient::new("https://httpbin.org/").unwrap();

    http_client.set_interceptor(|easy: &mut curl::easy::Easy| {
        let mut auth = Auth::new();
        auth.basic(true);
        easy.http_auth(&auth).unwrap();

        easy.username("me").unwrap();
        easy.password("secure").unwrap();
    });

    let response = block_on(http_client.get::<Response>("/basic-auth/me/secure")).unwrap();

    assert_eq!(response.user, "me");
    assert!(response.authenticated);
}
