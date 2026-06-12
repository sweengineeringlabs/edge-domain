//! SAF — event source service facade.
#[cfg(not(feature = "event"))]
pub use crate::api::event::Aggregate;
#[cfg(not(feature = "event"))]
pub use crate::api::event::ClosedEventSource;
#[cfg(not(feature = "event"))]
pub use crate::api::event::DomainEvent;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventEnvelope;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventError;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventReceiver;
#[cfg(not(feature = "event"))]
pub use crate::api::event::ExpectedVersion;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const EVENT_SOURCE_SVC: () = ();
