//! `DirectCommandBus` — SEA Rule 121 api/core mirror.
//!
//! `DirectCommandBus` is the api-layer marker type for the in-process command
//! bus. This path-level mirror lets the structural auditor match
//! `core/command/direct_command_bus.rs` to an api counterpart.
pub use crate::api::command::types::DirectCommandBus;
