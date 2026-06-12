//! Coverage for api/event/types/ins/in_process_event_bus.rs
use edge_domain::{Domain, EventBusConfig, InProcessEventBus};
use futures::executor::block_on;
use std::sync::Arc;

#[test]
fn test_in_process_event_bus_marker_type_is_constructible() {
    let _marker = InProcessEventBus;
}

#[test]
fn test_in_process_event_bus_factory_publishes_successfully() {
    block_on(async {
        use edge_domain::DomainEvent;
        struct AnyEvent;
        impl DomainEvent for AnyEvent {}
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        assert!(bus.publish(Arc::new(AnyEvent)).await.is_ok());
    });
}

#[test]
fn test_in_process_event_bus_subscriber_receives_published_event() {
    block_on(async {
        use edge_domain::DomainEvent;
        struct AnyEvent;
        impl DomainEvent for AnyEvent {}
        let bus = Domain::in_process_event_bus(EventBusConfig::default());
        let mut rx = bus.subscribe();
        assert!(bus.publish(Arc::new(AnyEvent)).await.is_ok());
        assert!(rx.recv().await.is_ok());
    });
}
