//! SAF — event bus service facade.
#[cfg(not(feature = "event"))]
pub use crate::api::EventBus;
#[cfg(not(feature = "event"))]
pub use crate::api::EventBusConfig;
#[cfg(not(feature = "event"))]
pub use crate::api::InProcessEventBus;
#[cfg(not(feature = "event"))]
pub use crate::api::NoopEventBus;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const EVENT_BUS_SVC: () = ();
