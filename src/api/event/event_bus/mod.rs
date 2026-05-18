//! `EventBus` — in-process fan-out contract.

#[allow(clippy::module_inception)]
pub mod event_bus;

pub use event_bus::EventBus;
