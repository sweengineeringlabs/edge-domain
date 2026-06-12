//! `DirectCommandBus` — marker type for the inline command bus.

/// Marker type describing a `CommandBus` that dispatches commands inline,
/// calling `cmd.execute()` directly in the same task with no queuing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectCommandBus;
