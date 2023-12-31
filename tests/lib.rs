use reqwest::{Client, StatusCode};
use wiremock::matchers::{header, header_exists, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock_logical_matchers::{and, not, or, xor};

#[tokio::test]
async fn test_and() {
    let mock_server = MockServer::start().await;

    Mock::given(and(header_exists("x-for-testing-purposes"), query_param("page", "1")))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let status_ok = Client::new()
        .get(&mock_server.uri())
        .header("x-for-testing-purposes", "42")
        .query(&[("page", "1")])
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

    let status_also_not_found = Client::new()
        .get(&mock_server.uri())
        .query(&[("page", "1")])
        .send()
        .await
        .unwrap()
        .status();

    let status_also_also_not_found = Client::new()
        .get(&mock_server.uri())
        .send()
        .await
        .unwrap()
        .status();

    assert_eq!(status_ok, StatusCode::OK);
    assert_eq!(status_not_found, StatusCode::NOT_FOUND);
    assert_eq!(status_also_not_found, StatusCode::NOT_FOUND);
    assert_eq!(status_also_also_not_found, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_or() {
    let mock_server = MockServer::start().await;

    Mock::given(or(header_exists("x-for-testing-purposes"), query_param("page", "1")))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let status_ok = Client::new()
        .get(&mock_server.uri())
        .header("x-for-testing-purposes", "42")
        .send()
        .await
        .unwrap()
        .status();

    let status_also_ok = Client::new()
        .get(&mock_server.uri())
        .query(&[("page", "1")])
        .send()
        .await
        .unwrap()
        .status();

    let status_also_also_ok = Client::new()
        .get(&mock_server.uri())
        .header("x-for-testing-purposes", "42")
        .query(&[("page", "1")])
        .send()
        .await
        .unwrap()
        .status();

    let status_not_found = Client::new()
        .get(&mock_server.uri())
        .send()
        .await
        .unwrap()
        .status();

    assert_eq!(status_ok, StatusCode::OK);
    assert_eq!(status_also_ok, StatusCode::OK);
    assert_eq!(status_also_also_ok, StatusCode::OK);
    assert_eq!(status_not_found, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_xor() {
    let mock_server = MockServer::start().await;

    Mock::given(xor(header_exists("x-for-testing-purposes"), query_param("page", "1")))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let status_ok = Client::new()
        .get(&mock_server.uri())
        .header("x-for-testing-purposes", "42")
        .send()
        .await
        .unwrap()
        .status();

    let status_also_ok = Client::new()
        .get(&mock_server.uri())
        .query(&[("page", "1")])
        .send()
        .await
        .unwrap()
        .status();

    let status_not_found = Client::new()
        .get(&mock_server.uri())
        .header("x-for-testing-purposes", "42")
        .query(&[("page", "1")])
        .send()
        .await
        .unwrap()
        .status();

    let status_also_not_found = Client::new()
        .get(&mock_server.uri())
        .send()
        .await
        .unwrap()
        .status();

    assert_eq!(status_ok, StatusCode::OK);
    assert_eq!(status_also_ok, StatusCode::OK);
    assert_eq!(status_not_found, StatusCode::NOT_FOUND);
    assert_eq!(status_also_not_found, StatusCode::NOT_FOUND);
}

#[tokio::test]
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

#[tokio::test]
async fn test_complex_expression() {
    let mock_server = MockServer::start().await;

    Mock::given(or(
        and(header_exists("x-for-testing-purposes"), query_param("page", "1")),
        header("x-bypass", "true"),
    ))
    .respond_with(ResponseTemplate::new(200))
    .mount(&mock_server)
    .await;

    let status_ok = Client::new()
        .get(&mock_server.uri())
        .header("x-bypass", "true")
        .send()
        .await
        .unwrap()
        .status();

    let status_also_ok = Client::new()
        .get(&mock_server.uri())
        .header("x-for-testing-purposes", "42")
        .query(&[("page", "1")])
        .send()
        .await
        .unwrap()
        .status();

    assert_eq!(status_ok, StatusCode::OK);
    assert_eq!(status_also_ok, StatusCode::OK);
}
