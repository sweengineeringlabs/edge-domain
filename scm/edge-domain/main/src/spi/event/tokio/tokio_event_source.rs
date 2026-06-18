//! [`TokioEventSource`] — tokio broadcast subscriber adapter.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::broadcast;

use crate::api::EventError;
use crate::api::DomainEvent;
use crate::api::EventSource;

/// Tokio broadcast [`EventSource`] — the subscriber side of
/// [`TokioEventBus`](super::tokio_event_bus::TokioEventBus).
///
/// Wraps a [`broadcast::Receiver`] and maps its receive errors onto the neutral
/// [`EventError`] surface so that no tokio type escapes through `api/`.
pub(crate) struct TokioEventSource(pub(crate) broadcast::Receiver<Arc<dyn DomainEvent>>);

impl EventSource for TokioEventSource {
    fn recv_next(&mut self) -> BoxFuture<'_, Result<Arc<dyn DomainEvent>, EventError>> {
        Box::pin(async move {
            match self.0.recv().await {
                Ok(event) => Ok(event),
                Err(broadcast::error::RecvError::Closed) => {
                    Err(EventError::Unavailable("event bus closed".into()))
                }
                Err(broadcast::error::RecvError::Lagged(n)) => Err(EventError::BroadcastLagged(n)),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    struct TokioEventSourceEvent;
    impl DomainEvent for TokioEventSourceEvent {
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

    #[tokio::test]
    async fn test_recv_next_returns_published_event() {
        let (tx, rx) = broadcast::channel::<Arc<dyn DomainEvent>>(4);
        let mut source = TokioEventSource(rx);
        assert!(tx.send(Arc::new(TokioEventSourceEvent)).is_ok());
        assert_eq!(source.recv_next().await.unwrap().event_type(), "test.any");
    }

    #[tokio::test]
    async fn test_recv_next_returns_unavailable_when_sender_dropped() {
        let (tx, rx) = broadcast::channel::<Arc<dyn DomainEvent>>(4);
        let mut source = TokioEventSource(rx);
        drop(tx);
        assert!(matches!(
            source.recv_next().await,
            Err(EventError::Unavailable(_))
        ));
    }
}
