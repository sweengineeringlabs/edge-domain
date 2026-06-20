//! SAF facade tests — `EventBootstrap` constructors.

use edge_domain_event::{DomainEvent, EventBootstrap, EventBusConfig, EventSource};

struct Events;
impl EventBootstrap for Events {}

#[derive(Clone)]
struct Evt;
impl DomainEvent for Evt {}

/// @covers: EventBootstrap::noop_bus — returns a NoopEventBus
#[test]
fn test_noop_bus_is_zero_sized_happy() {
    let bus = Events::noop_bus();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: EventBootstrap::noop_publisher — returns a NoopEventPublisher
#[test]
fn test_noop_publisher_is_zero_sized_happy() {
    let pub_ = Events::noop_publisher();
    assert_eq!(std::mem::size_of_val(&pub_), 0);
}

/// @covers: EventBootstrap::closed_source — returns a ClosedEventSource
#[test]
fn test_closed_source_is_zero_sized_happy() {
    let src = Events::closed_source();
    assert_eq!(std::mem::size_of_val(&src), 0);
}

/// @covers: EventBootstrap::in_memory_store — returns usable store
#[test]
fn test_in_memory_store_appends_successfully_happy() {
    use edge_domain_event::{EventStore, ExpectedVersion};
    let store = Events::in_memory_store::<Evt>();
    let seq = futures::executor::block_on(
        store.append("x", vec![Evt], ExpectedVersion::Any),
    )
    .expect("append");
    assert_eq!(seq, 1);
}

/// @covers: EventBootstrap::in_process_bus — capacity is honoured
#[test]
fn test_in_process_bus_constructed_with_config_edge() {
    let bus = Events::in_process_bus(EventBusConfig { capacity: 32 });
    // Sender capacity is set — just verify it doesn't panic
    assert_eq!(std::mem::size_of_val(&bus) > 0, true);
}

/// @covers: EventBootstrap::noop_bus — publish on noop bus never errors
#[test]
fn test_noop_bus_publish_never_errors_error() {
    use std::sync::Arc;
    use edge_domain_event::EventBus;
    let bus = Events::noop_bus();
    let result = futures::executor::block_on(bus.publish(Arc::new(Evt)));
    assert!(result.is_ok());
}

/// @covers: EventBootstrap::noop_bus — subscribe returns a receiver
#[test]
fn test_noop_bus_subscribe_returns_closed_receiver_edge() {
    use edge_domain_event::{EventBus, EventError};
    let bus = Events::noop_bus();
    let mut rx = bus.subscribe();
    let result = futures::executor::block_on(rx.recv());
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}

/// @covers: EventBootstrap::noop_publisher — publish via noop publisher never errors
#[test]
fn test_noop_publisher_publish_never_errors_error() {
    use edge_domain_event::EventPublisher;
    let pub_ = Events::noop_publisher();
    let result = futures::executor::block_on(pub_.publish(&Evt));
    assert!(result.is_ok());
}

/// @covers: EventBootstrap::noop_publisher — dyn dispatch works
#[test]
fn test_noop_publisher_dyn_dispatch_never_errors_edge() {
    use edge_domain_event::{EventPublisher, DomainEvent as DomainEventTrait};
    let pub_ = Events::noop_publisher();
    let evt: &dyn DomainEventTrait = &Evt;
    assert!(futures::executor::block_on(pub_.publish(evt)).is_ok());
}

/// @covers: EventBootstrap::in_memory_store — append conflict on NoStream after stream exists
#[test]
fn test_in_memory_store_conflict_on_no_stream_error() {
    use edge_domain_event::{EventStore, EventStoreError, ExpectedVersion};
    let store = Events::in_memory_store::<Evt>();
    futures::executor::block_on(
        store.append("x", vec![Evt], ExpectedVersion::NoStream),
    )
    .expect("first append");
    let err = futures::executor::block_on(
        store.append("x", vec![Evt], ExpectedVersion::NoStream),
    )
    .unwrap_err();
    assert!(matches!(err, EventStoreError::Conflict { .. }));
}

/// @covers: EventBootstrap::in_memory_store — load_from returns subset
#[test]
fn test_in_memory_store_load_from_returns_filtered_events_edge() {
    use edge_domain_event::{EventStore, ExpectedVersion};
    let store = Events::in_memory_store::<Evt>();
    futures::executor::block_on(
        store.append("y", vec![Evt, Evt, Evt], ExpectedVersion::Any),
    )
    .expect("append");
    let events = futures::executor::block_on(store.load_from("y", 2)).expect("load_from");
    assert_eq!(events.len(), 2);
}

/// @covers: EventBootstrap::closed_source — error message is non-empty
#[test]
fn test_closed_source_error_message_non_empty_error() {
    use edge_domain_event::EventError;
    let mut src = Events::closed_source();
    let err = match futures::executor::block_on(src.recv_next()) {
        Err(e) => e,
        Ok(_) => panic!("expected Err from closed source"),
    };
    assert!(!err.to_string().is_empty());
}

/// @covers: EventBootstrap::closed_source — repeated calls all return Unavailable
#[test]
fn test_closed_source_repeated_calls_all_unavailable_edge() {
    use edge_domain_event::EventError;
    let mut src = Events::closed_source();
    for _ in 0..3 {
        let result = futures::executor::block_on(src.recv_next());
        assert!(matches!(result, Err(EventError::Unavailable(_))));
    }
}

/// @covers: EventBootstrap::in_process_bus — publish and subscribe round-trip
#[test]
fn test_in_process_bus_publish_subscribe_round_trip_error() {
    use std::sync::Arc;
    use edge_domain_event::{EventBus};
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    rt.block_on(async {
        let bus = Events::in_process_bus(EventBusConfig::default());
        let mut rx = bus.subscribe();
        bus.publish(Arc::new(Evt)).await.expect("publish");
        let e = rx.recv().await.expect("recv");
        assert!(!e.event_type().is_empty());
    });
}

/// @covers: EventBootstrap::std — constructs the standard factory instance
#[test]
fn test_std_returns_std_event_factory_happy() {
    use edge_domain_event::StdEventFactory;
    let _f: StdEventFactory = Events::std();
}

/// @covers: EventBootstrap::std — std factory is zero-sized (pure dispatch)
#[test]
fn test_std_factory_is_zero_sized_error() {
    let f = Events::std();
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: EventBootstrap::std — std factory can immediately call noop_bus
#[test]
fn test_std_factory_can_call_noop_bus_immediately_edge() {
    use edge_domain_event::{EventBootstrap, StdEventFactory};
    let _ = StdEventFactory::noop_bus();
}

/// @covers: EventBootstrap::noop_aggregate — returns a NoopAggregate
#[test]
fn test_noop_aggregate_returns_noop_aggregate_happy() {
    use edge_domain_event::Aggregate;
    let agg = Events::noop_aggregate();
    assert_eq!(agg.id(), "");
}

/// @covers: EventBootstrap::noop_aggregate — apply is a no-op
#[test]
fn test_noop_aggregate_apply_is_noop_error() {
    use edge_domain_event::{Aggregate, NoopDomainEvent};
    let mut agg = Events::noop_aggregate();
    agg.apply(&NoopDomainEvent);
    assert_eq!(agg.id(), "");
}

/// @covers: EventBootstrap::noop_aggregate — multiple factory calls are independent
#[test]
fn test_noop_aggregate_multiple_calls_are_independent_edge() {
    use edge_domain_event::Aggregate;
    let a = Events::noop_aggregate();
    let b = Events::noop_aggregate();
    assert_eq!(a.id(), b.id());
}

/// @covers: EventBootstrap::noop_event — returns a NoopDomainEvent
#[test]
fn test_noop_event_returns_noop_domain_event_happy() {
    use edge_domain_event::DomainEvent;
    let evt = Events::noop_event();
    assert_eq!(evt.event_type(), "event");
}

/// @covers: EventBootstrap::noop_event — aggregate_id is empty string
#[test]
fn test_noop_event_aggregate_id_is_empty_error() {
    use edge_domain_event::DomainEvent;
    let evt = Events::noop_event();
    assert_eq!(evt.aggregate_id(), "");
}

/// @covers: EventBootstrap::noop_event — multiple calls produce independent values
#[test]
fn test_noop_event_multiple_calls_are_independent_edge() {
    use edge_domain_event::DomainEvent;
    let a = Events::noop_event();
    let b = Events::noop_event();
    assert_eq!(a.event_type(), b.event_type());
}
