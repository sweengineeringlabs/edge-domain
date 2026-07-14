//! Integration tests for [`EventBus`], [`EventSource`], and [`EventBusConfig`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::time::SystemTime;

use edge_application::DomainRuntime;
use edge_application::{
    Domain, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventBusConfig,
    EventBusPublishRequest, EventBusSubscribeRequest, EventError, EventOccurredAtRequest,
    EventOccurredAtResponse, EventSource, EventSourceRecvNextRequest, EventTypeRequest,
    EventTypeResponse,
};
use edge_application::{InProcessEventBusRequest, NoopEventBusRequest};

// ── test fixtures ────────────────────────────────────────────────────────────

#[derive(Clone)]
struct OrderCreated {
    order_id: String,
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
            occurred_at: SystemTime::now(),
        })
    }
}

// ── EventBusConfig ────────────────────────────────────────────────────────────

/// @covers: EventBusConfig
#[test]
fn test_event_bus_config_default_capacity_is_1024() {
    assert_eq!(EventBusConfig::default().capacity, 1024);
}

// ── in_process_event_bus ───────────────────────────────────────────────────────────

/// @covers: in_process_event_bus
#[tokio::test]
async fn test_in_process_event_bus_subscribe_then_publish_delivers_event() {
    let bus = Domain
        .in_process_event_bus(InProcessEventBusRequest {
            config: EventBusConfig::default(),
        })
        .unwrap()
        .bus;
    let mut rx: Box<dyn EventSource> = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    bus.publish(EventBusPublishRequest {
        event: Arc::new(OrderCreated {
            order_id: "ord-1".into(),
        }),
    })
    .await
    .unwrap();
    let event = rx
        .recv_next(EventSourceRecvNextRequest)
        .await
        .unwrap()
        .event;
    assert_eq!(
        event.event_type(EventTypeRequest).unwrap().event_type,
        "order.created"
    );
    assert_eq!(
        event
            .aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        "ord-1"
    );
}

/// @covers: in_process_event_bus
#[tokio::test]
async fn test_in_process_event_bus_publish_with_no_subscribers_returns_ok() {
    let bus = Domain
        .in_process_event_bus(InProcessEventBusRequest {
            config: EventBusConfig::default(),
        })
        .unwrap()
        .bus;
    let result = bus
        .publish(EventBusPublishRequest {
            event: Arc::new(OrderCreated {
                order_id: "ord-1".into(),
            }),
        })
        .await;
    assert!(result.is_ok());
}

/// @covers: in_process_event_bus
#[tokio::test]
async fn test_in_process_event_bus_fan_out_delivers_to_all_subscribers() {
    let bus = Domain
        .in_process_event_bus(InProcessEventBusRequest {
            config: EventBusConfig::default(),
        })
        .unwrap()
        .bus;
    let mut rx1 = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    let mut rx2 = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    bus.publish(EventBusPublishRequest {
        event: Arc::new(OrderCreated {
            order_id: "ord-2".into(),
        }),
    })
    .await
    .unwrap();
    assert_eq!(
        rx1.recv_next(EventSourceRecvNextRequest)
            .await
            .unwrap()
            .event
            .aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        "ord-2"
    );
    assert_eq!(
        rx2.recv_next(EventSourceRecvNextRequest)
            .await
            .unwrap()
            .event
            .aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        "ord-2"
    );
}

/// @covers: in_process_event_bus
#[tokio::test]
async fn test_in_process_event_bus_clone_shares_channel() {
    let bus = Domain
        .in_process_event_bus(InProcessEventBusRequest {
            config: EventBusConfig::default(),
        })
        .unwrap()
        .bus;
    let clone = bus.clone();
    let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    clone
        .publish(EventBusPublishRequest {
            event: Arc::new(OrderCreated {
                order_id: "ord-3".into(),
            }),
        })
        .await
        .unwrap();
    assert_eq!(
        rx.recv_next(EventSourceRecvNextRequest)
            .await
            .unwrap()
            .event
            .aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        "ord-3"
    );
}

// ── noop_event_bus ────────────────────────────────────────────────────────────

/// @covers: noop_event_bus
#[tokio::test]
async fn test_noop_event_bus_publish_returns_ok() {
    let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
    assert!(bus
        .publish(EventBusPublishRequest {
            event: Arc::new(OrderCreated {
                order_id: "ord-1".into()
            }),
        })
        .await
        .is_ok());
}

/// @covers: noop_event_bus
#[tokio::test]
async fn test_noop_event_bus_subscribe_returns_closed_receiver() {
    let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
    let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    assert!(matches!(
        rx.recv_next(EventSourceRecvNextRequest).await,
        Err(EventError::Unavailable(_))
    ));
}

// ── EventSource::recv_next ────────────────────────────────────────────────────

/// @covers: EventSource::recv_next
#[tokio::test]
async fn test_event_receiver_recv_returns_event_in_order() {
    let bus = Domain
        .in_process_event_bus(InProcessEventBusRequest {
            config: EventBusConfig::default(),
        })
        .unwrap()
        .bus;
    let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    for i in 0u32..3 {
        let id = format!("ord-{i}");
        bus.publish(EventBusPublishRequest {
            event: Arc::new(OrderCreated {
                order_id: id.clone(),
            }),
        })
        .await
        .unwrap();
        let event = rx
            .recv_next(EventSourceRecvNextRequest)
            .await
            .unwrap()
            .event;
        assert_eq!(
            event
                .aggregate_id(EventAggregateIdRequest)
                .unwrap()
                .aggregate_id,
            id
        );
    }
}
