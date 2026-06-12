#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventSource (via EventReceiver) is accessible from the crate root.

use edge_domain::Domain;
use edge_domain::EventBus;
use edge_domain::EventReceiver;

#[tokio::test]
async fn test_event_source_svc_facade_noop_receiver_returns_unavailable() {
    let bus = Domain::noop_event_bus();
    let mut rx: EventReceiver = bus.subscribe();
    let result = rx.recv().await;
    assert!(result.is_err());
}
