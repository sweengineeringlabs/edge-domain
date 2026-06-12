//! SAF — event store service facade.
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventStore;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventStoreError;
#[cfg(not(feature = "event"))]
pub use crate::api::event::InMemoryEventStore;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const EVENT_STORE_SVC: () = ();
