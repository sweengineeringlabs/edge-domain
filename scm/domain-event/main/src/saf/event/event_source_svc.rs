pub use crate::api::event::errors::EventError;
pub use crate::api::event::traits::EventSource;
pub use crate::api::event::types::ClosedEventSource;
pub use crate::api::event::types::EventReceiver;

/// Service name token for the event source port contract.
pub const EVENT_SOURCE_SVC: &str = "event_source";
