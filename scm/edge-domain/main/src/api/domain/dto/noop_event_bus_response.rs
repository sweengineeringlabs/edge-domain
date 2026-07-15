//! [`NoopEventBusResponse`] — a constructed no-op [`EventBus`](crate::api::EventBus) handle.

use std::sync::Arc;

use crate::api::EventBus;

/// The [`EventBus`] constructed by [`DomainRuntime::noop_event_bus`](crate::api::DomainRuntime::noop_event_bus).
pub struct NoopEventBusResponse {
    /// The discarding event bus.
    pub bus: Arc<dyn EventBus>,
}
