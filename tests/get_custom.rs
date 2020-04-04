use futures_executor::block_on;
use http_client::HttpClient;

#[test]
fn test_get() {
    struct Response {
        location: String,
    }

    let url = url::Url::parse("https://httpbin.org/").unwrap();
    let http_client = HttpClient::new(url.as_ref()).unwrap();

    let params = vec![("Location", "test")];

    let request = http_client.new_request_with_params(vec!["response-headers"], params);

    let response = block_on(http_client.perform_request::<Response, _>(
        request,
        |_request, response| {
            let mut location = None;

            for header in response.headers.iter() {
                if header.starts_with("location: ") {
                    let (_, value) = header.split_at("Location: ".len());
                    location = Some(value);
                    break;
                }
            }

            Ok(Response {
                location: location
                    .expect("Location header wasn't received")
                    .to_string(),
            })
        },
    ))
    .unwrap();

    assert_eq!(response.location, "test");
}
