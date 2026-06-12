//! API-layer type for the direct (in-process) query bus.

/// Marker type describing a `QueryBus` that dispatches queries inline,
/// calling `query.execute()` directly in the same task with no queuing.
///
/// The concrete implementation lives in `core::query::direct_query_bus`.
pub struct DirectQueryBus;
