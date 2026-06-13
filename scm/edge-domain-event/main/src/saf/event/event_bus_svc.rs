pub use crate::api::event::traits::EventBus;
pub use crate::api::event::types::EventBusConfig;
pub use crate::api::event::types::InProcessEventBus;
pub use crate::api::event::types::NoopEventBus;

/// Service name token for the event bus port contract.
pub const EVENT_BUS_SVC: &str = "event_bus";
