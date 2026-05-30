//! Extension hook trait for downstream consumers.

/// Marker trait for downstream domain extensions.
///
/// Implement this on a zero-size type to register custom cross-cutting
/// behaviour (audit hooks, logging adapters, etc.) that wraps domain
/// operations without altering public contracts.
pub trait DomainExtension: Send + Sync {}
