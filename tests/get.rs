use http_client::HttpClient;
use serde_derive::Deserialize;

#[test]
fn test_get() {
    #[derive(Deserialize)]
    struct Response {
        url: String,
    }

    let http_client = HttpClient::new("https://httpbin.org/").unwrap();

    assert_eq!(
        http_client.get::<Response>("/get").unwrap().url,
        "https://httpbin.org/get"
    );
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
    let response = http_client
        .get_with_params::<Response, _, _, _>("/get", params)
        .unwrap();

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
