//! End-to-end contract tests for the `HandlerBootstrap` trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{
    BootstrapNameRequest, BootstrapNameResponse, HandlerBootstrap, HandlerBuildResponse,
    HandlerError,
};

struct Cfg {
    valid: bool,
}

struct BootstrapDouble;

impl HandlerBootstrap for BootstrapDouble {
    fn bootstrap_name(
        &self,
        _req: BootstrapNameRequest,
    ) -> Result<BootstrapNameResponse, HandlerError> {
        Ok(BootstrapNameResponse {
            name: "bootstrap_double",
        })
    }

    type Config = Cfg;

    fn build(cfg: Cfg) -> Result<HandlerBuildResponse<Self>, HandlerError> {
        if cfg.valid {
            Ok(HandlerBuildResponse {
                handler: BootstrapDouble,
            })
        } else {
            Err(HandlerError::InvalidRequest("invalid config".into()))
        }
    }
}

/// @covers: HandlerBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let b = BootstrapDouble;
    assert!(!b
        .bootstrap_name(BootstrapNameRequest)
        .unwrap()
        .name
        .is_empty());
}

/// @covers: HandlerBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_matches_expected_value_edge() {
    let b = BootstrapDouble;
    assert_eq!(
        b.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "bootstrap_double"
    );
}

/// @covers: HandlerBootstrap::build
#[test]
fn test_build_valid_config_returns_ok_happy() {
    let result = BootstrapDouble::build(Cfg { valid: true });
    let handler = result.unwrap().handler;
    assert_eq!(
        handler.bootstrap_name(BootstrapNameRequest).unwrap().name,
        "bootstrap_double"
    );
}

/// @covers: HandlerBootstrap::build
#[test]
fn test_build_invalid_config_returns_err_error() {
    let result = BootstrapDouble::build(Cfg { valid: false });
    assert!(matches!(result, Err(HandlerError::InvalidRequest(_))));
}

/// @covers: HandlerBootstrap::build
#[test]
fn test_build_wraps_handler_in_response_edge() {
    let response = BootstrapDouble::build(Cfg { valid: true }).unwrap();
    assert_eq!(
        response
            .handler
            .bootstrap_name(BootstrapNameRequest)
            .unwrap()
            .name,
        "bootstrap_double"
    );
}
