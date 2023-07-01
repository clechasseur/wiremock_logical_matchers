//! Additional [wiremock] [matchers](Match) that implement logical operators.
//!
//! # Installing
//!
//! Add [wiremock_logical_matchers](crate) to your development dependencies:
//!
//! ```toml
//! [dev-dependencies]
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
//! use wiremock::matchers::{header, header_exists, path, query_param};
//! use wiremock_logical_matchers::{and, not, or, xor};
//!
//! #[async_std::test]
//! async fn test_getting_started() {
//!     let mock_server = MockServer::start().await;
//!
//!     Mock::given(path("/test"))
//!         .and(
//!             and(
//!                 header_exists("x-for-testing-purposes"),
//!                 query_param("page", "1")
//!             )
//!         ).and(
//!             or(
//!                 header("authorization", "Bearer some_token"),
//!                 query_param("override-security", "1")
//!             )
//!         ).and(
//!             xor(
//!                 header("x-license", "MIT"),
//!                 header("x-license-file", "LICENSE")
//!             )
//!         ).and(
//!             not(
//!                 header_exists("x-voldemort")
//!             )
//!         ).respond_with(ResponseTemplate::new(200))
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

use derivative::Derivative;
use wiremock::{Match, Request};

/// Shorthand for [AndMatcher].
pub fn and<L, R>(left_matcher: L, right_matcher: R) -> AndMatcher<L, R>
where
    L: Match,
    R: Match
{
    AndMatcher::new(left_matcher, right_matcher)
}

/// Shorthand for [OrMatcher].
pub fn or<L, R>(left_matcher: L, right_matcher: R) -> OrMatcher<L, R>
where
    L: Match,
    R: Match
{
    OrMatcher::new(left_matcher, right_matcher)
}

/// Shorthand for [XorMatcher].
pub fn xor<L, R>(left_matcher: L, right_matcher: R) -> XorMatcher<L, R>
where
    L: Match,
    R: Match
{
    XorMatcher::new(left_matcher, right_matcher)
}

/// Shorthand for [NotMatcher].
pub fn not<M>(matcher: M) -> NotMatcher<M>
where
    M: Match
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
/// use wiremock::{Mock, MockServer, ResponseTemplate};
/// use wiremock::matchers::{header_exists, query_param};
/// use wiremock_logical_matchers::and;
///
/// #[async_std::test]
/// async fn test_and() {
///     let mock_server = MockServer::start().await;
///
///     Mock::given(
///             and(
///                 header_exists("x-for-testing-purposes"),
///                 query_param("page", "1")
///             )
///         ).respond_with(ResponseTemplate::new(200))
///         .mount(&mock_server)
///         .await;
///
///     // ...
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
///             and(
///                 header_exists("x-for-testing-purposes"),
///                 query_param("page", "1")
///             ),
///             header("x-bypass", "true")
///         )
///     ).respond_with(ResponseTemplate::new(200))
///     .mount(&mock_server)
///     .await;
///
///     // ...
/// }
/// ```
///
/// # See also
///
/// [and]
#[derive(Derivative)]
#[derivative(Debug(bound = "L: Match + std::fmt::Debug, R: Match + std::fmt::Debug"))]
pub struct AndMatcher<L, R>(L, R)
where
    L: Match,
    R: Match;

impl<L, R> AndMatcher<L, R>
where
    L: Match,
    R: Match
{
    /// Creates a new `AND` matcher with the two given submatchers.
    ///
    /// # Arguments
    ///
    /// - `left_matcher` - First submatcher that must accept the request.
    /// - `right_matcher` - Second submatcher that must accept the request.
    ///                     Called only if the first submatcher also accepts the request.
    ///
    /// # See also
    ///
    /// [and]
    pub fn new(left_matcher: L, right_matcher: R) -> Self {
        Self(left_matcher, right_matcher)
    }
}

impl<L, R> Match for AndMatcher<L, R>
where
    L: Match,
    R: Match
{
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
/// use wiremock::{Mock, MockServer, ResponseTemplate};
/// use wiremock::matchers::{header, query_param};
/// use wiremock_logical_matchers::or;
///
/// #[async_std::test]
/// async fn test_or() {
///     let mock_server = MockServer::start().await;
///
///     Mock::given(
///             or(
///                 header("authorization", "Bearer some_token"),
///                 query_param("override-security", "1")
///             )
///         ).respond_with(ResponseTemplate::new(200))
///         .mount(&mock_server)
///         .await;
///
///     // ...
/// }
/// ```
///
/// # See also
///
/// [or]
#[derive(Derivative)]
#[derivative(Debug(bound = "L: Match + std::fmt::Debug, R: Match + std::fmt::Debug"))]
pub struct OrMatcher<L, R>(L, R)
where
    L: Match,
    R: Match;

impl<L, R> OrMatcher<L, R>
where
    L: Match,
    R: Match
{
    /// Creates a new `OR` matcher with the two given submatchers.
    ///
    /// # Arguments
    ///
    /// - `left_matcher` - First submatcher that can accept the request.
    /// - `right_matcher` - Second submatcher that can accept the request.
    ///                     Called only if the first submatcher does not accept the request.
    ///
    /// # See also
    ///
    /// [or]
    pub fn new(left_matcher: L, right_matcher: R) -> Self {
        Self(left_matcher, right_matcher)
    }
}

impl<L, R> Match for OrMatcher<L, R>
where
    L: Match,
    R: Match
{
    fn matches(&self, request: &Request) -> bool {
        self.0.matches(request) || self.1.matches(request)
    }
}

/// Match a request if exactly one submatcher accepts it.
///
/// # Example
///
/// ```
/// use wiremock::{Mock, MockServer, ResponseTemplate};
/// use wiremock::matchers::header;
/// use wiremock_logical_matchers::xor;
///
/// #[async_std::test]
/// async fn test_xor() {
///     let mock_server = MockServer::start().await;
///
///     Mock::given(
///             xor(
///                 header("x-license", "MIT"),
///                 header("x-license-file", "LICENSE")
///             )
///         ).respond_with(ResponseTemplate::new(200))
///         .mount(&mock_server)
///         .await;
///
///     // ...
/// }
/// ```
///
/// # See also
///
/// [xor]
#[derive(Derivative)]
#[derivative(Debug(bound = "L: Match + std::fmt::Debug, R: Match + std::fmt::Debug"))]
pub struct XorMatcher<L, R>(L, R)
where
    L: Match,
    R: Match;

impl<L, R> XorMatcher<L, R>
where
    L: Match,
    R: Match
{
    /// Creates a new `XOR` (exclusive `OR`) matcher with the two given submatchers.
    ///
    /// # Arguments
    ///
    /// - `left_matcher` - First submatcher that can accept the request.
    /// - `right_matcher` - Second submatcher that can accept the request.
    ///
    /// # See also
    ///
    /// [xor]
    pub fn new(left_matcher: L, right_matcher: R) -> Self {
        Self(left_matcher, right_matcher)
    }
}

impl<L, R> Match for XorMatcher<L, R>
where
    L: Match,
    R: Match
{
    fn matches(&self, request: &Request) -> bool {
        self.0.matches(request) != self.1.matches(request)
    }
}

/// Match a request if the submatcher does not accept it.
///
/// # Example
///
/// ```
/// use wiremock::{Mock, MockServer, ResponseTemplate};
/// use wiremock::matchers::header_exists;
/// use wiremock_logical_matchers::not;
///
/// #[async_std::test]
/// async fn test_not() {
///     let mock_server = MockServer::start().await;
///
///     Mock::given(
///             not(
///                 header_exists("x-voldemort")
///             )
///         ).respond_with(ResponseTemplate::new(200))
///         .mount(&mock_server)
///         .await;
///
///     // ...
/// }
/// ```
///
/// # See also
///
/// [not]
#[derive(Derivative)]
#[derivative(Debug(bound = "M: Match + std::fmt::Debug"))]
pub struct NotMatcher<M>(M)
where
    M: Match;

impl<M> NotMatcher<M>
where
    M: Match
{
    /// Creates a new `NOT` matcher with the given submatcher.
    ///
    /// # Arguments
    ///
    /// - `matcher` - Submatcher that must not accept the request.
    ///
    /// # See also
    ///
    /// [not]
    pub fn new(matcher: M) -> Self {
        Self(matcher)
    }
}

impl<M> Match for NotMatcher<M>
where
    M: Match
{
    fn matches(&self, request: &Request) -> bool {
        !self.0.matches(request)
    }
}
