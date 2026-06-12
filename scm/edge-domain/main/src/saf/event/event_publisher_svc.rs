//! SAF — event publisher service facade.
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventPublisher;
#[cfg(not(feature = "event"))]
pub use crate::api::event::NoopEventPublisher;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const EVENT_PUBLISHER_SVC: () = ();
