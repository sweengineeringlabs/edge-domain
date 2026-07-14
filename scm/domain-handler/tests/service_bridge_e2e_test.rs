//! End-to-end contract tests for the `ServiceBridge` marker trait, exercised through a
//! test-double implementation via the crate's public API.

use std::sync::Arc;

use edge_application_handler::ServiceBridge;

struct BridgeDouble;
impl ServiceBridge for BridgeDouble {}

fn is_send_sync<T: Send + Sync>() -> bool {
    let _marker: std::marker::PhantomData<T> = std::marker::PhantomData;
    true
}

/// @covers: ServiceBridge
#[test]
fn test_service_bridge_impl_is_send_and_sync_happy() {
    assert!(is_send_sync::<BridgeDouble>());
}

/// @covers: ServiceBridge
#[test]
fn test_service_bridge_coerces_to_trait_object_edge() {
    let b: Box<dyn ServiceBridge> = Box::new(BridgeDouble);
    assert_eq!(
        std::mem::size_of_val(&*b),
        std::mem::size_of::<BridgeDouble>()
    );
}

/// @covers: ServiceBridge
#[test]
fn test_service_bridge_storable_in_arc_collection_error() {
    let bridges: Vec<Arc<dyn ServiceBridge>> = vec![Arc::new(BridgeDouble), Arc::new(BridgeDouble)];
    assert_eq!(bridges.len(), 2);
}
