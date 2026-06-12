//! SAF — event bus service facade.
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventBus;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventBusConfig;
#[cfg(not(feature = "event"))]
pub use crate::api::event::InProcessEventBus;
#[cfg(not(feature = "event"))]
pub use crate::api::event::NoopEventBus;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const EVENT_BUS_SVC: () = ();
