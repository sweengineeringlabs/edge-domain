#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventSource is accessible from the crate root.

use edge_application::Domain;
use edge_application::DomainRuntime;
use edge_application::EventBus;
use edge_application::EventBusSubscribeRequest;
use edge_application::EventSource;
use edge_application::EventSourceRecvNextRequest;
use edge_application::NoopEventBusRequest;

#[tokio::test]
async fn test_event_source_svc_facade_noop_receiver_returns_unavailable() {
    let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
    let mut rx: Box<dyn EventSource> = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    let result = rx.recv_next(EventSourceRecvNextRequest).await;
    assert!(result.is_err());
}
