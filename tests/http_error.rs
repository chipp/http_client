use futures_executor::block_on;
use http_client::{Error, HttpClient, HttpMethod};
use serde_derive::Deserialize;

#[test]
fn test_404() {
    #[derive(Debug, Deserialize)]
    struct Response;

    let http_client = HttpClient::new("https://httpbin.org/").unwrap();

    match block_on(http_client.get::<Response, _>(vec!["status", "404"])).unwrap_err() {
        Error::HttpError(request, response) => {
            assert_eq!(request.url.as_str(), "https://httpbin.org/status/404");
            assert_eq!(request.method, HttpMethod::Get);
            assert_eq!(response.status_code, 404)
        }
        error => panic!(
            r#"assertion failed:
expected: `Error::HttpError`
     got: `{:?}`"#,
            error
        ),
    }
}
