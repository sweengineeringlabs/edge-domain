//! SAF — event store service facade.
#[cfg(not(feature = "event"))]
pub use crate::api::EventStore;
#[cfg(not(feature = "event"))]
pub use crate::api::EventStoreError;
#[cfg(not(feature = "event"))]
pub use crate::api::InMemoryEventStore;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const EVENT_STORE_SVC: () = ();
