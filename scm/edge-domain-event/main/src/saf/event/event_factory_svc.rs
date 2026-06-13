pub use crate::api::event::traits::EventFactory;
pub use crate::api::event::types::DefaultEventFactory;
pub use crate::api::event::types::StdEventFactory;

/// Service name token for the event factory port contract.
pub const EVENT_FACTORY_SVC: &str = "event_factory";
