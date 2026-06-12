#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — EventPublisher is exported from the crate root.

use edge_domain::Domain;
use edge_domain::DomainEvent;
use edge_domain::EventPublisher;

struct Pulse;
impl DomainEvent for Pulse {
    fn event_type(&self) -> &str {
        "pulse"
    }
    fn aggregate_id(&self) -> &str {
        "sys"
    }
}

#[tokio::test]
async fn test_event_publisher_svc_facade_noop_publish_returns_ok() {
    let p = Domain::noop_event_publisher();
    assert!(p.publish(&Pulse).await.is_ok());
}
