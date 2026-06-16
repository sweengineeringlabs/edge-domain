//! `EventBus` trait — publish/subscribe event bus contract.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::event::errors::EventError;
use crate::api::event::traits::DomainEvent;
use crate::api::event::types::EventReceiver;

/// A publish/subscribe event bus.
///
/// Callers publish [`DomainEvent`] instances and receive an [`EventReceiver`]
/// handle that produces subsequent events asynchronously.
pub trait EventBus: Send + Sync {
    /// Publish an event to all current subscribers.
    fn publish(&self, event: Arc<dyn DomainEvent>) -> BoxFuture<'_, Result<(), EventError>>;

    /// Subscribe, returning a stream-like [`EventReceiver`].
    fn subscribe(&self) -> EventReceiver;
}
