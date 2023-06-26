use reqwest::{Client, StatusCode};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::header_exists;
use wiremock_logical_matchers::not;

#[async_std::test]
async fn test_not() {
    let mock_server = MockServer::start().await;

    Mock::given(not(header_exists("x-for-testing-purposes")))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let status_ok = Client::new()
        .get(&mock_server.uri())
        .send()
        .await
        .unwrap()
        .status();

    let status_not_found = Client::new()
        .get(&mock_server.uri())
        .header("x-for-testing-purposes", "42")
        .send()
        .await
        .unwrap()
        .status();

    assert_eq!(status_ok, StatusCode::OK);
    assert_eq!(status_not_found, StatusCode::NOT_FOUND);
}
