//! Additional [wiremock] [matchers](Match) that implement logical operators.
//!
//! # Installing
//!
//! Add [wiremock_logical_matchers](crate) to your development dependencies:
//!
//! ```toml
//! [dev-dependencies]
//! # ...
//! wiremock = "0.5.19"
//! wiremock_logical_matchers = "0.1.0"
//! ```
//!
//! or by running:
//!
//! ```bash
//! cargo add wiremock_logical_matchers --dev
//! ```
//!
//! # Getting started
//!
//! ```
//! use wiremock::{Mock, MockServer, ResponseTemplate};
//! use wiremock::matchers::{bearer_token, header, header_exists, path, query_param};
//! use wiremock_logical_matchers::{and, not, or, xor};
//!
//! #[async_std::test]
//! async fn test_getting_started() {
//!     let mock_server = MockServer::start().await;
//!
//!     Mock::given(path("/test"))
//!         .and(and(header_exists("x-for-testing-purposes"), query_param("page", "1")))
//!         .and(or(bearer_token("some_token"), query_param("override-security", "1")))
//!         .and(xor(header("x-license", "MIT"), header("x-license-file", "LICENSE")))
//!         .and(not(header_exists("x-voldemort")))
//!         .respond_with(ResponseTemplate::new(200))
//!         .mount(&mock_server)
//!         .await;
//!
//!     // ...
//! }
//! ```
//!
//! # See also
//!
//! [wiremock on crates.io](https://crates.io/crates/wiremock)

use wiremock::{Match, Request};

/// Shorthand for [AndMatcher].
pub fn and<'a, 'b, L, R>(left_matcher: L, right_matcher: R) -> AndMatcher<'a, 'b>
where
    L: Match + 'a,
    R: Match + 'b,
{
    AndMatcher::new(left_matcher, right_matcher)
}

/// Shorthand for [OrMatcher].
pub fn or<'a, 'b, L, R>(left_matcher: L, right_matcher: R) -> OrMatcher<'a, 'b>
where
    L: Match + 'a,
    R: Match + 'b,
{
    OrMatcher::new(left_matcher, right_matcher)
}

/// Shorthand for [XorMatcher].
pub fn xor<'a, 'b, L, R>(left_matcher: L, right_matcher: R) -> XorMatcher<'a, 'b>
where
    L: Match + 'a,
    R: Match + 'b,
{
    XorMatcher::new(left_matcher, right_matcher)
}

/// Shorthand for [NotMatcher].
pub fn not<'a, M>(matcher: M) -> NotMatcher<'a>
where
    M: Match + 'a,
{
    NotMatcher::new(matcher)
}

/// Match a request if both submatchers accept it.
///
/// This matcher is shortcircuiting: if the first submatcher does not
/// accept the request, the second submatcher is not called.
///
/// # Example
///
/// ```
/// use reqwest::{Client, StatusCode};
/// use wiremock::{Mock, MockServer, ResponseTemplate};
/// use wiremock::matchers::{header_exists, path, query_param};
/// use wiremock_logical_matchers::and;
///
/// #[async_std::test]
/// async fn test_and() {
///     let mock_server = MockServer::start().await;
///
///     Mock::given(and(header_exists("x-for-testing-purposes"), query_param("page", "1")))
///         .respond_with(ResponseTemplate::new(200))
///         .mount(&mock_server)
///         .await;
///
///     let status_ok = Client::new()
///         .get(&mock_server.uri())
///         .header("x-for-testing-purposes", "42")
///         .query(&[("page", "1")])
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_not_found = Client::new()
///         .get(&mock_server.uri())
///         .header("x-for-testing-purposes", "42")
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_also_not_found = Client::new()
///         .get(&mock_server.uri())
///         .query(&[("page", "1")])
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_also_also_not_found = Client::new()
///         .get(&mock_server.uri())
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     assert_eq!(status_ok, StatusCode::OK);
///     assert_eq!(status_not_found, StatusCode::NOT_FOUND);
///     assert_eq!(status_also_not_found, StatusCode::NOT_FOUND);
///     assert_eq!(status_also_also_not_found, StatusCode::NOT_FOUND);
/// }
/// ```
///
/// # Notes
///
/// Because wiremock already supports an [and](wiremock::MockBuilder#method.and) method
/// on the [MockBuilder](wiremock::MockBuilder) type, there's no need to use this matcher
/// for simple `AND` cases. However, you might need to use it in more complex expressions
/// involving `OR`, for example:
///
/// ```
/// use reqwest::{Client, StatusCode};
/// use wiremock::{Mock, MockServer, ResponseTemplate};
/// use wiremock::matchers::{header, header_exists, query_param};
/// use wiremock_logical_matchers::{and, or};
///
/// #[async_std::test]
/// async fn test_complex_expression() {
///     let mock_server = MockServer::start().await;
///
///     Mock::given(
///         or(
///             and(header_exists("x-for-testing-purposes"), query_param("page", "1")),
///             header("x-bypass", "true")
///         )
///     ).respond_with(ResponseTemplate::new(200))
///     .mount(&mock_server)
///     .await;
///
///     let status_ok = Client::new()
///         .get(&mock_server.uri())
///         .header("bypass", "true")
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_also_ok = Client::new()
///         .get(&mock_server.uri())
///         .header("x-for-testing-purposes", "42")
///         .query(&[("page", "1")])
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     assert_eq!(status_ok, StatusCode::OK);
///     assert_eq!(status_also_ok, StatusCode::OK);
/// }
/// ```
pub struct AndMatcher<'a, 'b>(Box<dyn Match + 'a>, Box<dyn Match + 'b>);

impl<'a, 'b> AndMatcher<'a, 'b> {
    /// Creates a new `AND` matcher with the two given submatchers.
    ///
    /// # Arguments
    ///
    /// - `left_matcher` - First submatcher that must accept the request.
    /// - `right_matcher` - Second submatcher that must accept the request.
    ///                     Called only if the first submatcher also accepts the request.
    pub fn new<L, R>(left_matcher: L, right_matcher: R) -> Self
    where
        L: Match + 'a,
        R: Match + 'b,
    {
        Self(Box::new(left_matcher), Box::new(right_matcher))
    }
}

impl<'a, 'b> Match for AndMatcher<'a, 'b> {
    fn matches(&self, request: &Request) -> bool {
        self.0.matches(request) && self.1.matches(request)
    }
}

/// Match a request if either submatchers accept it.
///
/// This matcher is shortcircuiting: if the first submatcher accepts
/// the request, the second submatcher is not called.
///
/// # Example
///
/// ```
/// use reqwest::{Client, StatusCode};
/// use wiremock::{Mock, MockServer, ResponseTemplate};
/// use wiremock::matchers::{header_exists, path, query_param};
/// use wiremock_logical_matchers::or;
///
/// #[async_std::test]
/// async fn test_or() {
///     let mock_server = MockServer::start().await;
///
///     Mock::given(or(header_exists("x-for-testing-purposes"), query_param("page", "1")))
///         .respond_with(ResponseTemplate::new(200))
///         .mount(&mock_server)
///         .await;
///
///     let status_ok = Client::new()
///         .get(&mock_server.uri())
///         .header("x-for-testing-purposes", "42")
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_also_ok = Client::new()
///         .get(&mock_server.uri())
///         .query(&[("page", "1")])
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_also_also_ok = Client::new()
///         .get(&mock_server.uri())
///         .header("x-for-testing-purposes", "42")
///         .query(&[("page", "1")])
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_not_found = Client::new()
///         .get(&mock_server.uri())
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     assert_eq!(status_ok, StatusCode::OK);
///     assert_eq!(status_also_ok, StatusCode::OK);
///     assert_eq!(status_also_also_ok, StatusCode::OK);
///     assert_eq!(status_not_found, StatusCode::NOT_FOUND);
/// }
/// ```
pub struct OrMatcher<'a, 'b>(Box<dyn Match + 'a>, Box<dyn Match + 'b>);

impl<'a, 'b> OrMatcher<'a, 'b> {
    /// Creates a new `OR` matcher with the two given submatchers.
    ///
    /// # Arguments
    ///
    /// - `left_matcher` - First submatcher that can accept the request.
    /// - `right_matcher` - Second submatcher that can accept the request.
    ///                     Called only if the first submatcher does not accept the request.
    pub fn new<L, R>(left_matcher: L, right_matcher: R) -> Self
    where
        L: Match + 'a,
        R: Match + 'b,
    {
        Self(Box::new(left_matcher), Box::new(right_matcher))
    }
}

impl<'a, 'b> Match for OrMatcher<'a, 'b> {
    fn matches(&self, request: &Request) -> bool {
        self.0.matches(request) || self.1.matches(request)
    }
}

/// Match a request if exactly one submatcher accepts it.
///
/// # Example
///
/// ```
/// use reqwest::{Client, StatusCode};
/// use wiremock::{Mock, MockServer, ResponseTemplate};
/// use wiremock::matchers::{header_exists, path, query_param};
/// use wiremock_logical_matchers::xor;
///
/// #[async_std::test]
/// async fn test_xor() {
///     let mock_server = MockServer::start().await;
///
///     Mock::given(xor(header_exists("x-for-testing-purposes"), query_param("page", "1")))
///         .respond_with(ResponseTemplate::new(200))
///         .mount(&mock_server)
///         .await;
///
///     let status_ok = Client::new()
///         .get(&mock_server.uri())
///         .header("x-for-testing-purposes", "42")
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_also_ok = Client::new()
///         .get(&mock_server.uri())
///         .query(&[("page", "1")])
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_not_found = Client::new()
///         .get(&mock_server.uri())
///         .header("x-for-testing-purposes", "42")
///         .query(&[("page", "1")])
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_also_not_found = Client::new()
///         .get(&mock_server.uri())
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     assert_eq!(status_ok, StatusCode::OK);
///     assert_eq!(status_also_ok, StatusCode::OK);
///     assert_eq!(status_not_found, StatusCode::NOT_FOUND);
///     assert_eq!(status_also_not_found, StatusCode::NOT_FOUND);
/// }
/// ```
pub struct XorMatcher<'a, 'b>(Box<dyn Match + 'a>, Box<dyn Match + 'b>);

impl<'a, 'b> XorMatcher<'a, 'b> {
    /// Creates a new `XOR` (exclusive `OR`) matcher with the two given submatchers.
    ///
    /// # Arguments
    ///
    /// - `left_matcher` - First submatcher that can accept the request.
    /// - `right_matcher` - Second submatcher that can accept the request.
    pub fn new<L, R>(left_matcher: L, right_matcher: R) -> Self
    where
        L: Match + 'a,
        R: Match + 'b,
    {
        Self(Box::new(left_matcher), Box::new(right_matcher))
    }
}

impl<'a, 'b> Match for XorMatcher<'a, 'b> {
    fn matches(&self, request: &Request) -> bool {
        self.0.matches(request) != self.1.matches(request)
    }
}

/// Match a request if the submatcher does not accept it.
///
/// # Example
///
/// ```
/// use reqwest::{Client, StatusCode};
/// use wiremock::{Mock, MockServer, ResponseTemplate};
/// use wiremock::matchers::{header_exists, path, query_param};
/// use wiremock_logical_matchers::not;
///
/// #[async_std::test]
/// async fn test_not() {
///     let mock_server = MockServer::start().await;
///
///     Mock::given(not(header_exists("x-for-testing-purposes")))
///         .respond_with(ResponseTemplate::new(200))
///         .mount(&mock_server)
///         .await;
///
///     let status_ok = Client::new()
///         .get(&mock_server.uri())
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     let status_not_found = Client::new()
///         .get(&mock_server.uri())
///         .header("x-for-testing-purposes", "42")
///         .send()
///         .await
///         .unwrap()
///         .status();
///
///     assert_eq!(status_ok, StatusCode::OK);
///     assert_eq!(status_not_found, StatusCode::NOT_FOUND);
/// }
/// ```
pub struct NotMatcher<'a>(Box<dyn Match + 'a>);

impl<'a> NotMatcher<'a> {
    /// Creates a new `NOT` matcher with the given submatcher.
    ///
    /// # Arguments
    ///
    /// - `matcher` - Submatcher that must not accept the request.
    pub fn new<M>(matcher: M) -> Self
    where
        M: Match + 'a,
    {
        Self(Box::new(matcher))
    }
}

impl<'a> Match for NotMatcher<'a> {
    fn matches(&self, request: &Request) -> bool {
        !self.0.matches(request)
    }
}
