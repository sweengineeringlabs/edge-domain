#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventSource is accessible from the crate root.

use edge_domain::Domain;
use edge_domain::EventBus;
use edge_domain::EventBusSubscribeRequest;
use edge_domain::EventSource;
use edge_domain::EventSourceRecvNextRequest;

#[tokio::test]
async fn test_event_source_svc_facade_noop_receiver_returns_unavailable() {
    let bus = Domain::noop_event_bus();
    let mut rx: Box<dyn EventSource> = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    let result = rx.recv_next(EventSourceRecvNextRequest).await;
    assert!(result.is_err());
}
