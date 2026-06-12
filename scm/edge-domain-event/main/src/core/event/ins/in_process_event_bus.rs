//! [`EventBus`] impl for [`InProcessEventBus`] — tokio broadcast channel bus.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::event::errors::EventError;
use crate::api::event::traits::{DomainEvent, EventBus, EventSource};
use crate::api::event::types::{EventReceiver, InProcessEventBus};

/// Private receiver side of the broadcast channel.
struct InProcessEventBusSource {
    receiver: tokio::sync::broadcast::Receiver<Arc<dyn DomainEvent>>,
}

impl EventSource for InProcessEventBusSource {
    fn recv_next(&mut self) -> BoxFuture<'_, Result<Arc<dyn DomainEvent>, EventError>> {
        Box::pin(async {
            match self.receiver.recv().await {
                Ok(event) => Ok(event),
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    Err(EventError::Unavailable("channel closed".into()))
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                    Err(EventError::BroadcastLagged(n))
                }
            }
        })
    }
}

impl EventBus for InProcessEventBus {
    fn publish(&self, event: Arc<dyn DomainEvent>) -> BoxFuture<'_, Result<(), EventError>> {
        Box::pin(async move {
            // send() fails only when there are no receivers, which is not an error
            let _ = self.sender.send(event);
            Ok(())
        })
    }

    fn subscribe(&self) -> EventReceiver {
        EventReceiver::new(InProcessEventBusSource {
            receiver: self.sender.subscribe(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct InProcessEventBusTestEvt;
    impl DomainEvent for InProcessEventBusTestEvt {
        fn event_type(&self) -> &str {
            "test.evt"
        }
    }

    /// @covers: publish
    #[test]
    fn test_publish_no_subscribers_returns_ok_happy() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio rt");
        rt.block_on(async {
            let bus = InProcessEventBus::new(16);
            let result = bus.publish(Arc::new(InProcessEventBusTestEvt)).await;
            assert!(result.is_ok());
        });
    }

    /// @covers: publish
    #[test]
    fn test_publish_with_subscriber_delivers_event_happy() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio rt");
        rt.block_on(async {
            let bus = InProcessEventBus::new(16);
            let mut rx = bus.subscribe();
            bus.publish(Arc::new(InProcessEventBusTestEvt)).await.expect("publish");
            let event = rx.recv().await.expect("recv");
            assert_eq!(event.event_type(), "test.evt");
        });
    }

    /// @covers: subscribe
    #[test]
    fn test_subscribe_two_receivers_independent_edge() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio rt");
        rt.block_on(async {
            let bus = InProcessEventBus::new(16);
            let mut rx1 = bus.subscribe();
            let mut rx2 = bus.subscribe();
            bus.publish(Arc::new(InProcessEventBusTestEvt)).await.expect("publish");
            assert!(rx1.recv().await.is_ok());
            assert!(rx2.recv().await.is_ok());
        });
    }
}
