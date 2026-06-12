//! `DirectQueryBus` — marker type for the inline query bus.

/// Marker type describing a `QueryBus` that dispatches queries inline,
/// calling `query.execute()` directly in the same task with no queuing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectQueryBus;
