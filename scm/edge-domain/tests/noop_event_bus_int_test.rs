//! Coverage for api/event/types/noop/noop_event_bus.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]
use edge_domain::{Domain, EventBusPublishRequest, EventBusSubscribeRequest, NoopEventBus};
use futures::executor::block_on;
use std::sync::Arc;

#[test]
fn test_noop_event_bus_marker_type_is_constructible() {
    let _marker = NoopEventBus;
}

#[test]
fn test_noop_event_bus_publish_returns_ok() {
    block_on(async {
        use edge_domain::DomainEvent;
        struct AnyEvent;
        impl DomainEvent for AnyEvent {}
        let bus = Domain::noop_event_bus();
        assert!(bus
            .publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent)
            })
            .await
            .is_ok());
    });
}

#[test]
fn test_noop_event_bus_subscribe_returns_closed_receiver() {
    block_on(async {
        let bus = Domain::noop_event_bus();
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        // noop bus receiver immediately signals unavailable
        assert!(rx.recv().await.is_err());
    });
}
