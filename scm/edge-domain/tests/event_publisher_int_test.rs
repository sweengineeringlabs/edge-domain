//! Integration tests for `DomainEvent` and `EventPublisher`.
#![cfg(feature = "event")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
    EventOccurredAtRequest, EventOccurredAtResponse, EventPublisher, EventPublisherPublishRequest,
    EventTypeRequest, EventTypeResponse,
};
use futures::future::BoxFuture;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

struct OrderCreated {
    order_id: String,
    occurred_at: SystemTime,
}

impl DomainEvent for OrderCreated {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "order.created",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: &self.order_id,
        })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: self.occurred_at,
        })
    }
}

struct CountingPublisher {
    count: Arc<AtomicUsize>,
}

impl EventPublisher for CountingPublisher {
    fn publish(
        &self,
        _req: EventPublisherPublishRequest<'_>,
    ) -> BoxFuture<'_, Result<(), EventError>> {
        self.count.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }
}

struct FailingPublisher;

impl EventPublisher for FailingPublisher {
    fn publish(
        &self,
        _req: EventPublisherPublishRequest<'_>,
    ) -> BoxFuture<'_, Result<(), EventError>> {
        Box::pin(async { Err(EventError::Unavailable("bus down".into())) })
    }
}

/// @covers: DomainEvent::event_type, DomainEvent::aggregate_id, DomainEvent::occurred_at
#[test]
fn test_domain_event_trait_returns_correct_fields() {
    let evt = OrderCreated {
        order_id: "ord-1".into(),
        occurred_at: SystemTime::now(),
    };
    assert_eq!(
        evt.event_type(EventTypeRequest).unwrap().event_type,
        "order.created"
    );
    assert_eq!(
        evt.aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        "ord-1"
    );
    let _ = evt.occurred_at(EventOccurredAtRequest).unwrap();
}

/// @covers: EventPublisher::publish
#[tokio::test]
async fn test_event_publisher_trait_publish_increments_count_on_success() {
    let count = Arc::new(AtomicUsize::new(0));
    let publisher: Arc<dyn EventPublisher> = Arc::new(CountingPublisher {
        count: Arc::clone(&count),
    });
    let evt = OrderCreated {
        order_id: "ord-1".into(),
        occurred_at: SystemTime::now(),
    };
    publisher
        .publish(EventPublisherPublishRequest { event: &evt })
        .await
        .unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

/// @covers: EventPublisher::publish
#[tokio::test]
async fn test_event_publisher_trait_publish_propagates_error_on_failure() {
    let publisher: Arc<dyn EventPublisher> = Arc::new(FailingPublisher);
    let evt = OrderCreated {
        order_id: "ord-1".into(),
        occurred_at: SystemTime::now(),
    };
    assert!(publisher
        .publish(EventPublisherPublishRequest { event: &evt })
        .await
        .is_err());
}
