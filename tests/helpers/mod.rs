use std::fmt::Debug;
use wiremock::{Match, Request};

// The technique used here to test our matchers' Debug implementations was inspired by:
// https://stackoverflow.com/a/64957689/21308608

pub struct DebugTest<M>(pub M)
where
    M: Match;

pub trait DoesImplementDebug {
    fn debug_if_possible(&self) -> String;
}

impl<M> DoesImplementDebug for DebugTest<M>
where
    M: Match + Debug
{
    fn debug_if_possible(&self) -> String {
        format!("{:?}", self.0)
    }
}

pub trait DoesNotImplementDebug {
    fn debug_if_possible(&self) -> String;
}

impl<M> DoesNotImplementDebug for &DebugTest<M>
where
    M: Match
{
    fn debug_if_possible(&self) -> String {
        "".to_string()
    }
}

pub struct NotDebugMatcher(pub usize);

impl Match for NotDebugMatcher {
    fn matches(&self, _request: &Request) -> bool {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct DebugMatcher(pub usize);

impl Match for DebugMatcher {
    fn matches(&self, _request: &Request) -> bool {
        unimplemented!();
    }
}

#[macro_export]
macro_rules! test_dual_matcher_debug {
    ( $matcher:ident ) => {
        paste::paste! {
            mod [<$matcher:snake _debug_tests>] {
                use $crate::helpers::{DoesImplementDebug, DoesNotImplementDebug};
                use super::*;

                #[test]
                fn test_both_debug() {
                    let both_debug = $matcher::new($crate::helpers::DebugMatcher(42), $crate::helpers::DebugMatcher(23));
                    let debug_test = $crate::helpers::DebugTest(both_debug);
                    let actual = (&debug_test).debug_if_possible();
                    assert_eq!(actual.is_empty(), false,
                        "Expected {}'s Debug test to be true, but returned: {}", stringify!($matcher), actual);
                }

                #[test]
                fn test_left_debug() {
                    let left_debug = $matcher::new($crate::helpers::DebugMatcher(42), $crate::helpers::NotDebugMatcher(23));
                    let debug_test = $crate::helpers::DebugTest(left_debug);
                    let actual = (&debug_test).debug_if_possible();
                    assert_eq!(actual.is_empty(), true,
                        "Expected {}'s Debug test to be false, but returned: {}", stringify!($matcher), actual);
                }

                #[test]
                fn test_right_debug() {
                    let right_debug = $matcher::new($crate::helpers::NotDebugMatcher(42), $crate::helpers::DebugMatcher(23));
                    let debug_test = $crate::helpers::DebugTest(right_debug);
                    let actual = (&debug_test).debug_if_possible();
                    assert_eq!(actual.is_empty(), true,
                        "Expected {}'s Debug test to be false, but returned: {}", stringify!($matcher), actual);
                }

                #[test]
                fn test_no_debug() {
                    let no_debug = $matcher::new($crate::helpers::NotDebugMatcher(42), $crate::helpers::NotDebugMatcher(23));
                    let debug_test = $crate::helpers::DebugTest(no_debug);
                    let actual = (&debug_test).debug_if_possible();
                    assert_eq!(actual.is_empty(), true,
                        "Expected {}'s Debug test to be false, but returned: {}", stringify!($matcher), actual);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! test_single_matcher_debug {
    ( $matcher:ident ) => {
        paste::paste! {
            mod [<$matcher:snake _debug_tests>] {
                use $crate::helpers::{DoesImplementDebug, DoesNotImplementDebug};
                use super::*;

                #[test]
                fn test_with_debug() {
                    let with_debug = $matcher::new($crate::helpers::DebugMatcher(42));
                    let debug_test = $crate::helpers::DebugTest(with_debug);
                    let actual = (&debug_test).debug_if_possible();
                    assert_eq!(actual.is_empty(), false,
                        "Expected {}'s Debug test to be true, but returned: {}", stringify!($matcher), actual);
                }

                #[test]
                fn test_without_debug() {
                    let without_debug = $matcher::new($crate::helpers::NotDebugMatcher(23));
                    let debug_test = $crate::helpers::DebugTest(without_debug);
                    let actual = (&debug_test).debug_if_possible();
                    assert_eq!(actual.is_empty(), true,
                        "Expected {}'s Debug test to be false, but returned: {}", stringify!($matcher), actual);
                }
            }
        }
    }
}
