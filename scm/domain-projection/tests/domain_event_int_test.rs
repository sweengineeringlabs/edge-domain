//! Integration coverage for the `edge-domain-event` dependency used in projection impls.
// @allow: no_mocks_in_integration

use edge_domain_event::DomainEvent;

#[derive(Clone)]
struct DomainEventTestEvt;

impl DomainEvent for DomainEventTestEvt {}

#[test]
fn test_domain_event_default_event_type_returns_event_happy() {
    let e = DomainEventTestEvt;
    assert_eq!(e.event_type(), "event");
}

#[test]
fn test_domain_event_default_aggregate_id_returns_empty_error() {
    let e = DomainEventTestEvt;
    assert_eq!(e.aggregate_id(), "");
}

#[test]
fn test_domain_event_occurred_at_returns_system_time_edge() {
    let e = DomainEventTestEvt;
    let t = e.occurred_at();
    assert!(t.elapsed().is_ok());
}
