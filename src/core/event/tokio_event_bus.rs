//! [`TokioEventBus`] — in-process fan-out bus backed by [`tokio::sync::broadcast`].

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::broadcast;

use crate::api::event::domain_event::DomainEvent;
use crate::api::event::event_bus::EventBus;
use crate::api::event::event_bus_config::EventBusConfig;
use crate::api::error::EventError;
use crate::api::types::EventReceiver;

/// In-process event bus backed by a tokio broadcast channel.
///
/// Topics are all events (there is no per-topic routing). All active
/// subscribers receive every event published.  Slow subscribers that fall
/// behind by more than `capacity` events receive
/// [`EventError::BroadcastLagged`] on their next [`EventReceiver::recv`].
///
/// `TokioEventBus` is cheaply cloneable — clone produces another handle to
/// the same channel.
#[derive(Clone)]
pub(crate) struct TokioEventBus {
    sender: broadcast::Sender<Arc<dyn DomainEvent>>,
}

impl TokioEventBus {
    pub(crate) fn new(config: EventBusConfig) -> Self {
        let (sender, _) = broadcast::channel(config.capacity);
        Self { sender }
    }
}

impl EventBus for TokioEventBus {
    fn publish(&self, event: Arc<dyn DomainEvent>) -> BoxFuture<'_, Result<(), EventError>> {
        let sender = self.sender.clone();
        Box::pin(async move {
            // Silently drop if there are no active subscribers.
            let _ = sender.send(event);
            Ok(())
        })
    }

    fn subscribe(&self) -> EventReceiver {
        EventReceiver(self.sender.subscribe())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    struct TokioEventBusTestEvent;
    impl DomainEvent for TokioEventBusTestEvent {
        fn event_type(&self) -> &str {
            "test.any"
        }
        fn aggregate_id(&self) -> &str {
            "id"
        }
        fn occurred_at(&self) -> SystemTime {
            SystemTime::now()
        }
    }

    #[test]
    fn test_new_creates_bus_with_configured_capacity() {
        let bus = TokioEventBus::new(EventBusConfig { capacity: 32 });
        drop(bus);
    }

    #[tokio::test]
    async fn test_subscribe_then_publish_delivers_event() {
        let bus = TokioEventBus::new(EventBusConfig::default());
        let mut rx = bus.subscribe();
        bus.publish(Arc::new(TokioEventBusTestEvent)).await.unwrap();
        let event = rx.recv().await.unwrap();
        assert_eq!(event.event_type(), "test.any");
    }

    #[tokio::test]
    async fn test_publish_with_no_subscribers_returns_ok() {
        let bus = TokioEventBus::new(EventBusConfig::default());
        assert!(bus.publish(Arc::new(TokioEventBusTestEvent)).await.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_subscribers_each_receive_event() {
        let bus = TokioEventBus::new(EventBusConfig::default());
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();
        bus.publish(Arc::new(TokioEventBusTestEvent)).await.unwrap();
        assert_eq!(rx1.recv().await.unwrap().event_type(), "test.any");
        assert_eq!(rx2.recv().await.unwrap().event_type(), "test.any");
    }

    #[tokio::test]
    async fn test_clone_shares_channel_with_original() {
        let bus = TokioEventBus::new(EventBusConfig::default());
        let clone = bus.clone();
        let mut rx = bus.subscribe();
        clone
            .publish(Arc::new(TokioEventBusTestEvent))
            .await
            .unwrap();
        assert!(rx.recv().await.is_ok());
    }
}
