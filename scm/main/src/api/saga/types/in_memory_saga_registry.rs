//! API interface surface for the in-memory saga registry implementation.
//!
//! SEA api/ counterpart for `core/saga/in_memory_saga_registry.rs`.
//! The implementation is accessed via
//! [`Domain::new_in_memory_saga_registry`](crate::Domain::new_in_memory_saga_registry).

/// API marker type for the in-memory saga registry.
///
/// The concrete implementation lives in `core::saga::in_memory_saga_registry`
/// and stores sagas in a `HashMap` keyed by their `SagaId`.  Consumers obtain
/// it via the SAF factory
/// [`Domain::new_in_memory_saga_registry`](crate::Domain::new_in_memory_saga_registry).
pub struct InMemorySagaRegistry;
