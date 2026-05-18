//! [`EventReceiver`] — subscriber-side handle for the in-process event bus.

use std::sync::Arc;

use tokio::sync::broadcast;

use crate::api::event::domain_event::DomainEvent;
use crate::api::event::event_error::EventError;

/// Subscriber-side handle for receiving events from an [`crate::EventBus`].
///
/// Obtain an `EventReceiver` by calling [`crate::EventBus::subscribe`] and poll
/// it via [`recv`](EventReceiver::recv).  Each receiver has its own independent
/// queue — events are delivered to every active subscriber.
pub struct EventReceiver(pub(crate) broadcast::Receiver<Arc<dyn DomainEvent>>);

impl EventReceiver {
    /// Wait for the next event.
    ///
    /// # Errors
    ///
    /// - [`EventError::Unavailable`] — the bus was dropped (no more events).
    /// - [`EventError::BroadcastLagged`] — the subscriber fell too far behind;
    ///   some messages were dropped.  The count indicates how many were skipped.
    pub async fn recv(&mut self) -> Result<Arc<dyn DomainEvent>, EventError> {
        match self.0.recv().await {
            Ok(event) => Ok(event),
            Err(broadcast::error::RecvError::Closed) => {
                Err(EventError::Unavailable("event bus closed".into()))
            }
            Err(broadcast::error::RecvError::Lagged(n)) => Err(EventError::BroadcastLagged(n)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: recv
    #[test]
    fn test_recv_closed_channel_returns_unavailable() {
        let (tx, rx) = broadcast::channel::<Arc<dyn DomainEvent>>(4);
        drop(tx);
        let mut receiver = EventReceiver(rx);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let result = rt.block_on(receiver.recv());
        assert!(matches!(result, Err(EventError::Unavailable(_))));
    }
}
