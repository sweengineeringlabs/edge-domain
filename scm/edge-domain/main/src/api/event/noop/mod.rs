//! Noop sub-theme — api/ counterpart for `core/event/noop` (SEA Rule 121).
//!
//! The marker types are declared in the parent `event` theme's `types/` so they
//! are anchored by the event trait contracts; this module re-exports them to
//! mirror the `core/event/noop` implementation submodule.

pub use crate::api::event::types::{NoopEventBus, NoopEventPublisher};
