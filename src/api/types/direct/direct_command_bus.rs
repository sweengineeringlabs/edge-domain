//! API-layer type for the direct (in-process) command bus.

/// Marker type describing a `CommandBus` that dispatches commands inline,
/// calling `cmd.execute()` directly in the same task with no queuing.
///
/// The concrete implementation lives in `core::command::direct_command_bus`.
#[allow(dead_code)]
pub struct DirectCommandBus;


