//! Coverage for api/event/types/closed_event_source.rs
use edge_application::ClosedEventSource;

#[test]
fn test_closed_event_source_is_constructible_happy() {
    let src = ClosedEventSource;
    assert_eq!(std::mem::size_of_val(&src), 0, "ClosedEventSource is a ZST");
}

#[test]
fn test_closed_event_source_zst_size_edge() {
    assert_eq!(std::mem::size_of::<ClosedEventSource>(), 0);
}

#[test]
fn test_closed_event_source_two_instances_are_independent_edge() {
    let a = ClosedEventSource;
    let b = ClosedEventSource;
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}
