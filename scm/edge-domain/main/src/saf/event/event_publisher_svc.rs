//! SAF — event publisher service facade.
#[cfg(not(feature = "event"))]
pub use crate::api::EventPublisher;
#[cfg(not(feature = "event"))]
pub use crate::api::NoopEventPublisher;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const EVENT_PUBLISHER_SVC: () = ();
