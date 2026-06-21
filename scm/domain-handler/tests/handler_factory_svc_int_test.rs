//! Integration tests — `HandlerBootstrap` trait.

use edge_domain_handler::{HandlerBootstrap, HandlerError};

struct Cfg {
    name: String,
    valid: bool,
}

#[derive(Debug)]
struct NamedHandler {
    name: String,
}

impl HandlerBootstrap for NamedHandler {
    fn bootstrap_name(&self) -> &'static str {
        "named_handler"
    }

    type Config = Cfg;

    fn build(cfg: Cfg) -> Result<Self, HandlerError> {
        if cfg.valid {
            Ok(NamedHandler { name: cfg.name })
        } else {
            Err(HandlerError::InvalidRequest("config invalid".into()))
        }
    }
}

/// @covers: HandlerBootstrap::build — valid config
#[test]
fn test_build_valid_config_returns_handler_happy() {
    let h = NamedHandler::build(Cfg {
        name: "worker".into(),
        valid: true,
    });
    assert!(h.is_ok());
    assert_eq!(h.unwrap().name, "worker");
}

/// @covers: HandlerBootstrap::build — invalid config
#[test]
fn test_build_invalid_config_returns_err_error() {
    let h = NamedHandler::build(Cfg {
        name: String::new(),
        valid: false,
    });
    assert!(h.is_err());
    assert!(matches!(h.unwrap_err(), HandlerError::InvalidRequest(_)));
}

/// @covers: HandlerBootstrap::build — empty name is still valid if flag is set
#[test]
fn test_build_empty_name_valid_flag_returns_ok_edge() {
    let h = NamedHandler::build(Cfg {
        name: String::new(),
        valid: true,
    });
    assert!(h.is_ok());
    assert!(h.unwrap().name.is_empty());
}
