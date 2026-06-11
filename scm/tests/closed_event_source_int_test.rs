//! Coverage for api/event/types/closed_event_source.rs
use edge_domain::ClosedEventSource;

#[test]
fn test_closed_event_source_is_constructible_happy() {
    let _src = ClosedEventSource;
}

#[test]
fn test_closed_event_source_zst_size_edge() {
    assert_eq!(std::mem::size_of::<ClosedEventSource>(), 0);
}

#[test]
fn test_closed_event_source_two_instances_are_independent_edge() {
    let _a = ClosedEventSource;
    let _b = ClosedEventSource;
}
