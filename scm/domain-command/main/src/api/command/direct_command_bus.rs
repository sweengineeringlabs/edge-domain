//! `DirectCommandBus` — SEA Rule 121 api/core mirror.

/// Inline command bus that dispatches commands synchronously without queuing.
pub type DirectCommandBus = crate::api::command::types::direct_command_bus::DirectCommandBus;
