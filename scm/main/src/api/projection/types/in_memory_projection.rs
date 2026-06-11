//! API interface surface for the in-memory projection implementation.
//!
//! SEA api/ counterpart for `core/projection/in_memory_projection.rs`.
//! The implementation is accessed via
//! [`Domain::new_in_memory_projection`](crate::Domain::new_in_memory_projection).

/// API marker type for the in-memory projection.
///
/// The concrete implementation lives in `core::projection::in_memory_projection`
/// and folds each [`EventEnvelope`](crate::EventEnvelope) into a read model via
/// a caller-supplied reducer.  Consumers obtain it via the SAF factory
/// [`Domain::new_in_memory_projection`](crate::Domain::new_in_memory_projection).
pub struct InMemoryProjection;
