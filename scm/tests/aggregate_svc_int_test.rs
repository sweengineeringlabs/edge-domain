#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Aggregate trait is exported from the crate root.

use edge_domain::Aggregate;
use edge_domain::DomainEvent;

#[derive(Clone)]
struct TestDomainEvent;
impl DomainEvent for TestDomainEvent {
    fn event_type(&self) -> &str {
        "test.event"
    }
    fn aggregate_id(&self) -> &str {
        "agg-1"
    }
}

#[derive(Default)]
struct TestAggregate {
    applied: u32,
}
impl Aggregate for TestAggregate {
    type Event = TestDomainEvent;
    fn apply(&mut self, _event: &Self::Event) {
        self.applied += 1;
    }
}

#[test]
fn test_aggregate_svc_facade_apply_increments_count() {
    let mut agg = TestAggregate::default();
    agg.apply(&TestDomainEvent);
    assert_eq!(agg.applied, 1);
}
