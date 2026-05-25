//! Tests for DomainEvent trait
use edge_domain::DomainEvent;
use std::time::SystemTime;

struct TestEvent;
impl DomainEvent for TestEvent {
    fn event_type(&self) -> &str {
        "test.event"
    }
    fn aggregate_id(&self) -> &str {
        "agg-1"
    }
    fn occurred_at(&self) -> SystemTime {
        SystemTime::now()
    }
}

#[test]
fn test_domain_event() {
    let e = TestEvent;
    assert_eq!(e.event_type(), "test.event");
    assert_eq!(e.aggregate_id(), "agg-1");
}
