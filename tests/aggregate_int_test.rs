//! Tests for Aggregate trait
use edge_domain::{Aggregate, DomainEvent};
use std::time::SystemTime;

#[derive(Default)]
struct TestAggregate {
    id: String,
}

struct TestEvent;
impl DomainEvent for TestEvent {
    fn event_type(&self) -> &str {
        "test"
    }
    fn aggregate_id(&self) -> &str {
        "1"
    }
    fn occurred_at(&self) -> SystemTime {
        SystemTime::now()
    }
}

impl Aggregate for TestAggregate {
    type Event = TestEvent;
    fn apply(&mut self, e: &TestEvent) {
        self.id = e.aggregate_id().into();
    }
    fn id(&self) -> &str {
        &self.id
    }
}

#[test]
fn test_aggregate_apply() {
    let mut agg = TestAggregate::default();
    agg.apply(&TestEvent);
    assert_eq!(agg.id(), "1");
}
