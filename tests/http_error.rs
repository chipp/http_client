use futures::executor::block_on;
use http_client::{Error, HttpClient};
use serde_derive::Deserialize;

#[test]
fn test_404() {
    #[derive(Debug, Deserialize)]
    struct Response;

    let http_client = HttpClient::new("https://httpbin.org/").unwrap();

    match block_on(http_client.get::<Response>("/status/404")).unwrap_err() {
        Error::HttpError(404) => (),
        error => panic!(
            r#"assertion failed:
expected: `Error::HttpError(404)`
     got: `{:?}`"#,
            error
        ),
    }
}
