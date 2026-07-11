//! `EchoHandler` impl path mirror — SEA Rule 121.
//!
//! This path-level mirror lets the structural auditor match
//! `core/handler/echo_handler.rs` (the `Handler` impl for
//! [`EchoHandler`](crate::api::handler::types::EchoHandler)) to an api
//! counterpart now that `core/handler/` no longer nests a `types/` subdirectory.

/// SEA Rule 121 marker — path co-location sentinel for
/// `core/handler/echo_handler.rs`.
pub(crate) const _RULE_121: () = ();
