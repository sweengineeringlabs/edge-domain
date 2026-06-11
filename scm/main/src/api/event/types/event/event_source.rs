//! [`EventSource`] — technology-neutral source of domain events.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::event::errors::EventError;
use crate::api::event::traits::domain_event::DomainEvent;

/// Technology-neutral source of domain events for a single subscriber.
///
/// Implemented by the backing channel adapter in the `spi/` layer. Keeping
/// this trait in `api/` lets [`EventReceiver`](super::event_receiver::EventReceiver)
/// stay free of any concrete channel technology.
pub trait EventSource: Send {
    /// Wait for the next event from the underlying channel.
    ///
    /// Returns [`EventError::Unavailable`] when the channel is closed and
    /// [`EventError::BroadcastLagged`] when the subscriber fell behind.
    fn recv_next(&mut self) -> BoxFuture<'_, Result<Arc<dyn DomainEvent>, EventError>>;
}
