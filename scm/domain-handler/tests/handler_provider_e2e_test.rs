//! End-to-end contract tests for the `HandlerProvider` trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{
    BootstrapNameRequest, HandlerProvider, HandlerRegistry, LenRequest, StdRegistryBridge,
};

struct ProviderDouble;
impl HandlerProvider for ProviderDouble {}

/// @covers: HandlerProvider::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let p = ProviderDouble;
    assert!(!p
        .bootstrap_name(BootstrapNameRequest)
        .unwrap()
        .name
        .is_empty());
}

/// @covers: HandlerProvider::bootstrap_name
#[test]
fn test_bootstrap_name_matches_expected_value_edge() {
    let p = ProviderDouble;
    assert_eq!(
        p.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "handler_provider"
    );
}

/// @covers: HandlerProvider::bootstrap_name — no error path exists; documents the absence
/// by asserting the second of two independent calls still returns the expected value.
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let p = ProviderDouble;
    let _first = p.bootstrap_name(BootstrapNameRequest).unwrap();
    assert_eq!(
        p.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "handler_provider"
    );
}

/// @covers: HandlerProvider::echo_handler
#[test]
fn test_echo_handler_creates_handler_with_id_and_pattern_happy() {
    let h = ProviderDouble::echo_handler("my-id", "/route");
    assert_eq!(h.id, "my-id");
    assert_eq!(h.pattern, "/route");
}

/// @covers: HandlerProvider::noop_handler_factory
#[test]
fn test_noop_handler_factory_constructs_instance_edge() {
    let f = ProviderDouble::noop_handler_factory();
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: HandlerProvider::in_process_registry
#[test]
fn test_in_process_registry_creates_empty_registry_happy() {
    let reg = ProviderDouble::in_process_registry::<String, String>();
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
}

/// @covers: HandlerProvider::default_bridge
#[test]
fn test_default_bridge_constructs_std_registry_bridge_happy() {
    let b: StdRegistryBridge = ProviderDouble::default_bridge();
    assert_eq!(format!("{b:?}"), "StdRegistryBridge");
}

/// @covers: HandlerProvider::default_bridge
#[test]
fn test_default_bridge_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<StdRegistryBridge>(), 0);
}
