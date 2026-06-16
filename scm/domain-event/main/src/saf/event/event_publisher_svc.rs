pub use crate::api::event::traits::EventPublisher;
pub use crate::api::event::types::NoopEventPublisher;

/// Service name token for the event publisher port contract.
pub const EVENT_PUBLISHER_SVC: &str = "event_publisher";
