//! `DirectCommandBus` — type alias re-exporting from the designated struct home.
/// Marker for an inline `CommandBus` that dispatches commands in the same task.
pub type DirectCommandBus = crate::api::command::types::DirectCommandBus;
