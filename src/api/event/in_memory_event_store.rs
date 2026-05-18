//! API interface surface for the in-memory event store implementation.
//!
//! SEA api/ counterpart for `core/event/in_memory_event_store.rs`.
//! The implementation is accessed via [`crate::new_in_memory_event_store`].

/// API marker type for the in-memory event store.
///
/// The concrete implementation lives in `core::event::in_memory_event_store`.
/// Consumers obtain it via the SAF factory [`crate::new_in_memory_event_store`].
#[allow(dead_code)]
pub struct InMemoryEventStore;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_event_store_api_marker_is_constructible() {
        let _ = InMemoryEventStore;
    }
}
