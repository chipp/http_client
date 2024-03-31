use chipp_http::{ErrorKind, HttpClient, HttpMethod};
use futures_executor::block_on;
use serde::Deserialize;

#[test]
fn test_404() {
    #[derive(Debug, Deserialize)]
    struct Response;

    let http_client = HttpClient::new("https://httpbin.org/").unwrap();
    let error = block_on(http_client.get::<Response, _>(vec!["status", "404"])).unwrap_err();

    match &error.kind {
        ErrorKind::HttpError(response) => {
            assert_eq!(error.request.url.as_str(), "https://httpbin.org/status/404");
            assert_eq!(error.request.method, HttpMethod::Get);
            assert_eq!(response.status_code, 404)
        }
        _ => panic!(
            r#"assertion failed:
expected: `ErrorKind::HttpError`
     got: `{:?}`"#,
            error
        ),
    }
}
