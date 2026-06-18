//! SAF — event source service facade.
#[cfg(not(feature = "event"))]
pub use crate::api::Aggregate;
#[cfg(not(feature = "event"))]
pub use crate::api::ClosedEventSource;
#[cfg(not(feature = "event"))]
pub use crate::api::DomainEvent;
#[cfg(not(feature = "event"))]
pub use crate::api::EventEnvelope;
#[cfg(not(feature = "event"))]
pub use crate::api::EventError;
#[cfg(not(feature = "event"))]
pub use crate::api::EventReceiver;
#[cfg(not(feature = "event"))]
pub use crate::api::ExpectedVersion;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const EVENT_SOURCE_SVC: () = ();
