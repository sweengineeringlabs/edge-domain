//! [`NoopEventPublisherResponse`] — a constructed no-op [`EventPublisher`](crate::api::EventPublisher) handle.

use std::sync::Arc;

use crate::api::EventPublisher;

/// The [`EventPublisher`] constructed by [`DomainRuntime::noop_event_publisher`](crate::api::DomainRuntime::noop_event_publisher).
pub struct NoopEventPublisherResponse {
    /// The discarding event publisher.
    pub publisher: Arc<dyn EventPublisher>,
}
