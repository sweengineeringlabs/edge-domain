//! Integration tests for `NoopAggregate`.

use edge_domain_event::{Aggregate, NoopAggregate, NoopDomainEvent};

/// @covers: NoopAggregate::id — returns empty string before any apply
#[test]
fn test_id_noop_aggregate_before_apply_returns_empty_happy() {
    let agg = NoopAggregate;
    assert_eq!(agg.id(), "");
}

/// @covers: NoopAggregate::apply — is a no-op; id stays empty after apply
#[test]
fn test_apply_noop_aggregate_after_apply_id_unchanged_happy() {
    let mut agg = NoopAggregate;
    agg.apply(&NoopDomainEvent);
    assert_eq!(agg.id(), "");
}

/// @covers: NoopAggregate — Default constructs without panic
#[test]
fn test_default_noop_aggregate_constructs_edge() {
    let _agg = NoopAggregate::default();
}

/// @covers: NoopAggregate::apply — repeated apply calls don't panic
#[test]
fn test_apply_noop_aggregate_repeated_calls_do_not_panic_edge() {
    let mut agg = NoopAggregate;
    for _ in 0..5 {
        agg.apply(&NoopDomainEvent);
    }
    assert_eq!(agg.id(), "");
}

/// @covers: NoopAggregate — satisfies Aggregate trait bounds (Send + Sync)
#[test]
fn test_noop_aggregate_satisfies_send_sync_bounds_happy() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NoopAggregate>();
}
