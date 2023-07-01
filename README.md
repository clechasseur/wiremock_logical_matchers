# wiremock_logical_matchers

[![CI](https://github.com/clechasseur/wiremock_logical_matchers/actions/workflows/ci.yml/badge.svg?branch=main&event=push)](https://github.com/clechasseur/wiremock_logical_matchers/actions/workflows/ci.yml) [![codecov](https://codecov.io/gh/clechasseur/wiremock_logical_matchers/branch/main/graph/badge.svg?token=NIW54Q8UC3)](https://codecov.io/gh/clechasseur/wiremock_logical_matchers) [![Security audit](https://github.com/clechasseur/wiremock_logical_matchers/actions/workflows/audit-check.yml/badge.svg?branch=main)](https://github.com/clechasseur/wiremock_logical_matchers/actions/workflows/audit-check.yml)<br/>
[![crates.io](https://img.shields.io/crates/v/wiremock_logical_matchers.svg)](https://crates.io/crates/wiremock_logical_matchers) [![downloads](https://img.shields.io/crates/d/wiremock_logical_matchers.svg)](https://crates.io/crates/wiremock_logical_matchers) [![docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/wiremock_logical_matchers)

Additional [matchers](https://docs.rs/wiremock/latest/wiremock/trait.Match.html) for [wiremock](https://crates.io/crates/wiremock) that implement logical operators (AND, OR, XOR, NOT).

# Installing

Add `wiremock_logical_matchers` to your development dependencies:

```toml
[dev-dependencies]
# ...
wiremock = "0.5.19"
wiremock_logical_matchers = "0.1.0"
```

or by running:

```bash
cargo add wiremock_logical_matchers --dev
```

# Getting started

```rust
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{header, header_exists, path, query_param};
use wiremock_logical_matchers::{and, not, or, xor};

#[async_std::test]
async fn test_getting_started() {
    let mock_server = MockServer::start().await;

    Mock::given(path("/test"))
        .and(
            and(
                header_exists("x-for-testing-purposes"),
                query_param("page", "1")
            )
        ).and(
            or(
                header("authorization", "Bearer some_token"),
                query_param("override-security", "1")
            )
        ).and(
            xor(
                header("x-license", "MIT"),
                header("x-license-file", "LICENSE")
            )
        ).and(
            not(
                header_exists("x-voldemort")
            )
        ).respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    // ...
}
```

# See also

[wiremock on crates.io](https://crates.io/crates/wiremock)
