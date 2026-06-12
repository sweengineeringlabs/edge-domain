//! `EventSource` trait — pull-based event source contract.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::event::errors::EventError;
use crate::api::event::traits::DomainEvent;

/// A pull-based source of [`DomainEvent`] values.
///
/// Implementors yield one event per `recv_next` call. When the source is
/// exhausted or closed they return [`EventError::Unavailable`].
pub trait EventSource: Send {
    /// Receive the next available event, waiting if necessary.
    fn recv_next(&mut self) -> BoxFuture<'_, Result<Arc<dyn DomainEvent>, EventError>>;
}
