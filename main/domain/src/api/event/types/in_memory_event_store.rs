//! API interface surface for the in-memory event store implementation.
//!
//! SEA api/ counterpart for `core/event/in_memory_event_store.rs`.
//! The implementation is accessed via [`Domain::new_in_memory_event_store`](crate::Domain::new_in_memory_event_store).

/// API marker type for the in-memory event store.
///
/// The concrete implementation lives in `core::event::in_memory_event_store`.
/// Consumers obtain it via the SAF factory [`Domain::new_in_memory_event_store`](crate::Domain::new_in_memory_event_store).
pub struct InMemoryEventStore;
