//! SEA api/ counterpart for `core/event/noop/`.
//!
//! Re-exports noop event types from the canonical `api/event/types/noop/` module.

pub use crate::api::event::types::noop::NoopEventBus;
pub use crate::api::event::types::noop::NoopEventPublisher;
