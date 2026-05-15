//! `Event` module — domain event contracts.

pub mod domain_event;
pub mod event_error;
pub mod event_publisher;
pub mod noop_event_publisher;

pub use domain_event::DomainEvent;
pub use event_error::EventError;
pub use event_publisher::EventPublisher;
