//! Noop saga infrastructure — zero-sized markers for testing and bring-up.

mod saga;
mod saga_command;
mod saga_event;

pub use saga::NoopSaga;
pub use saga_command::NoopSagaCommand;
pub use saga_event::NoopSagaEvent;
