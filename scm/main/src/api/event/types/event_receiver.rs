//! [`EventReceiver`] — subscriber-side handle for the in-process event bus.

use std::sync::Arc;

use crate::api::event::error::EventError;
use crate::api::event::traits::domain_event::DomainEvent;
use crate::api::event::types::event_source::EventSource;

/// Subscriber-side handle for receiving events from an [`crate::EventBus`].
///
/// Obtain an `EventReceiver` by calling [`crate::EventBus::subscribe`] and poll
/// it via [`recv`](EventReceiver::recv).  Each receiver has its own independent
/// queue — events are delivered to every active subscriber.
///
/// The handle is technology-neutral: it wraps a boxed `EventSource`, so the
/// backing channel implementation lives entirely in `spi/` and never leaks
/// into `api/`.
pub struct EventReceiver(Box<dyn EventSource>);

impl EventReceiver {
    /// Construct an `EventReceiver` from any `EventSource`.
    ///
    /// Used by event-bus implementations in `core/` and `spi/` to hand back a
    /// neutral subscriber handle.
    pub fn new(source: impl EventSource + 'static) -> Self {
        Self(Box::new(source))
    }

    /// Wait for the next event.
    ///
    /// # Errors
    ///
    /// - [`EventError::Unavailable`] — the bus was dropped (no more events).
    /// - [`EventError::BroadcastLagged`] — the subscriber fell too far behind;
    ///   some messages were dropped.  The count indicates how many were skipped.
    pub async fn recv(&mut self) -> Result<Arc<dyn DomainEvent>, EventError> {
        self.0.recv_next().await
    }
}
