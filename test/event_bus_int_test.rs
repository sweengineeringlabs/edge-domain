//! Integration tests for [`EventBus`], [`EventReceiver`], and [`EventBusConfig`].

use std::sync::Arc;
use std::time::SystemTime;

use edge_domain::{
    noop_event_bus, tokio_event_bus, DomainEvent, EventBusConfig, EventError, EventReceiver,
};

// ── test fixtures ────────────────────────────────────────────────────────────

#[derive(Clone)]
struct OrderCreated {
    order_id: String,
}

impl DomainEvent for OrderCreated {
    fn event_type(&self) -> &str {
        "order.created"
    }
    fn aggregate_id(&self) -> &str {
        &self.order_id
    }
    fn occurred_at(&self) -> SystemTime {
        SystemTime::now()
    }
}

// ── EventBusConfig ────────────────────────────────────────────────────────────

/// @covers: EventBusConfig
#[test]
fn test_event_bus_config_default_capacity_is_1024() {
    assert_eq!(EventBusConfig::default().capacity, 1024);
}

// ── tokio_event_bus ───────────────────────────────────────────────────────────

/// @covers: tokio_event_bus
#[tokio::test]
async fn test_tokio_event_bus_subscribe_then_publish_delivers_event() {
    let bus = tokio_event_bus(EventBusConfig::default());
    let mut rx: EventReceiver = bus.subscribe();
    bus.publish(Arc::new(OrderCreated {
        order_id: "ord-1".into(),
    }))
    .await
    .unwrap();
    let event = rx.recv().await.unwrap();
    assert_eq!(event.event_type(), "order.created");
    assert_eq!(event.aggregate_id(), "ord-1");
}

/// @covers: tokio_event_bus
#[tokio::test]
async fn test_tokio_event_bus_publish_with_no_subscribers_returns_ok() {
    let bus = tokio_event_bus(EventBusConfig::default());
    let result = bus
        .publish(Arc::new(OrderCreated {
            order_id: "ord-1".into(),
        }))
        .await;
    assert!(result.is_ok());
}

/// @covers: tokio_event_bus
#[tokio::test]
async fn test_tokio_event_bus_fan_out_delivers_to_all_subscribers() {
    let bus = tokio_event_bus(EventBusConfig::default());
    let mut rx1 = bus.subscribe();
    let mut rx2 = bus.subscribe();
    bus.publish(Arc::new(OrderCreated {
        order_id: "ord-2".into(),
    }))
    .await
    .unwrap();
    assert_eq!(rx1.recv().await.unwrap().aggregate_id(), "ord-2");
    assert_eq!(rx2.recv().await.unwrap().aggregate_id(), "ord-2");
}

/// @covers: tokio_event_bus
#[tokio::test]
async fn test_tokio_event_bus_clone_shares_channel() {
    let bus = tokio_event_bus(EventBusConfig::default());
    let clone = bus.clone();
    let mut rx = bus.subscribe();
    clone
        .publish(Arc::new(OrderCreated {
            order_id: "ord-3".into(),
        }))
        .await
        .unwrap();
    assert_eq!(rx.recv().await.unwrap().aggregate_id(), "ord-3");
}

// ── noop_event_bus ────────────────────────────────────────────────────────────

/// @covers: noop_event_bus
#[tokio::test]
async fn test_noop_event_bus_publish_returns_ok() {
    let bus = noop_event_bus();
    assert!(bus
        .publish(Arc::new(OrderCreated {
            order_id: "ord-1".into()
        }))
        .await
        .is_ok());
}

/// @covers: noop_event_bus
#[tokio::test]
async fn test_noop_event_bus_subscribe_returns_closed_receiver() {
    let bus = noop_event_bus();
    let mut rx = bus.subscribe();
    assert!(matches!(rx.recv().await, Err(EventError::Unavailable(_))));
}

// ── EventReceiver ─────────────────────────────────────────────────────────────

/// @covers: EventReceiver
#[tokio::test]
async fn test_event_receiver_recv_returns_event_in_order() {
    let bus = tokio_event_bus(EventBusConfig::default());
    let mut rx = bus.subscribe();
    for i in 0u32..3 {
        let id = format!("ord-{i}");
        bus.publish(Arc::new(OrderCreated {
            order_id: id.clone(),
        }))
        .await
        .unwrap();
        let event = rx.recv().await.unwrap();
        assert_eq!(event.aggregate_id(), id);
    }
}
