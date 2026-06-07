//! [`ClosedEventSource`] — an [`EventSource`] that yields no events.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::event::error::EventError;
use crate::api::event::traits::domain_event::DomainEvent;
use crate::api::event::types::event_source::EventSource;

/// [`EventSource`] that never yields an event — the channel is closed from the
/// outset, so the first
/// [`EventReceiver::recv`](crate::api::event::types::event_receiver::EventReceiver::recv)
/// returns [`EventError::Unavailable`].
pub(crate) struct ClosedEventSource;

impl EventSource for ClosedEventSource {
    fn recv_next(&mut self) -> BoxFuture<'_, Result<Arc<dyn DomainEvent>, EventError>> {
        Box::pin(async { Err(EventError::Unavailable("event bus closed".into())) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_recv_next_returns_unavailable() {
        let mut source = ClosedEventSource;
        assert!(matches!(
            source.recv_next().await,
            Err(EventError::Unavailable(_))
        ));
    }
}
