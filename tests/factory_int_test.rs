//! Integration tests for saf factory functions.

use std::sync::Arc;
use edge_domain::{new_handler_registry, HandlerRegistry};

/// @covers: new_handler_registry
#[test]
fn test_factory_fn_new_handler_registry_returns_empty_arc_registry() {
    let reg: Arc<HandlerRegistry<String, String>> = new_handler_registry();
    assert!(reg.is_empty());
    assert_eq!(reg.len(), 0);
}
