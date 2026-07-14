//! Coverage for api/event/errors/event/event_store_error.rs
use edge_application::EventStoreError;

#[test]
fn test_event_store_error_unavailable_carries_message() {
    let e = EventStoreError::Unavailable("db down".into());
    assert!(e.to_string().contains("db down"));
}

#[test]
fn test_event_store_error_conflict_carries_all_fields() {
    let e = EventStoreError::Conflict {
        aggregate_id: "agg-1".into(),
        expected: 1,
        actual: 3,
    };
    let s = e.to_string();
    assert!(s.contains("agg-1") && s.contains('1') && s.contains('3'));
}

#[test]
fn test_event_store_error_internal_carries_message() {
    let e = EventStoreError::Internal("unexpected".into());
    assert!(e.to_string().contains("unexpected"));
}
