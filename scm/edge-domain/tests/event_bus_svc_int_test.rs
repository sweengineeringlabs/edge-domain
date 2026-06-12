#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventBus is exported from the crate root.

use edge_domain::Domain;
use edge_domain::DomainEvent;
use edge_domain::EventBus;
use std::sync::Arc;

struct Tick;
impl DomainEvent for Tick {
    fn event_type(&self) -> &str {
        "tick"
    }
    fn aggregate_id(&self) -> &str {
        "sys"
    }
}

#[tokio::test]
async fn test_event_bus_svc_facade_noop_publish_returns_ok() {
    let bus = Domain::noop_event_bus();
    assert!(bus.publish(Arc::new(Tick)).await.is_ok());
}

#[test]
fn test_event_bus_svc_facade_subscribe_returns_receiver() {
    let bus = Domain::noop_event_bus();
    let _rx = bus.subscribe();
}
