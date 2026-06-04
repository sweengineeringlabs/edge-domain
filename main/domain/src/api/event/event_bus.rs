//! [`EventBus`] — in-process fan-out broadcast contract.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::error::EventError;
use crate::api::event::domain_event::DomainEvent;
use crate::api::types::EventReceiver;

/// In-process publish/subscribe bus for domain events.
///
/// All active subscribers receive every event published after they subscribed.
/// No events are buffered for subscribers that haven't subscribed yet.
///
/// Use `Arc<dyn EventBus>` to share a bus across handlers, background tasks,
/// and middleware:
///
/// ```rust,ignore
/// let bus: Arc<dyn EventBus> = Arc::new(tokio_event_bus(EventBusConfig::default()));
/// bus.publish(Arc::new(OrderCreated { .. })).await?;
/// let mut rx = bus.subscribe();
/// let event = rx.recv().await?;
/// ```
pub trait EventBus: Send + Sync {
    /// Broadcast `event` to all active subscribers.
    ///
    /// Silently succeeds when there are no active subscribers (fire-and-forget).
    fn publish(&self, event: Arc<dyn DomainEvent>) -> BoxFuture<'_, Result<(), EventError>>;

    /// Subscribe to all future events published on this bus.
    ///
    /// Returns a [`EventReceiver`] the caller polls via [`EventReceiver::recv`].
    /// Subscriptions are independent — each subscriber has its own queue.
    fn subscribe(&self) -> EventReceiver;
}
