//! Integration tests for `EchoProviderCompleter` — api/ type coverage.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::EchoProviderCompleter;

/// @covers: EchoProviderCompleter — type is constructible as a unit struct
#[test]
fn test_echo_provider_completer_constructs_happy() {
    let _c = EchoProviderCompleter;
}

/// @covers: EchoProviderCompleter — debug representation is non-empty
#[test]
fn test_echo_provider_completer_debug_non_empty_error() {
    let repr = format!("{:?}", EchoProviderCompleter);
    assert!(!repr.is_empty());
}

/// @covers: EchoProviderCompleter — copy semantics produce independent bindings
#[test]
fn test_echo_provider_completer_copy_edge() {
    let a = EchoProviderCompleter;
    let b = a;
    let _both = (a, b);
}
