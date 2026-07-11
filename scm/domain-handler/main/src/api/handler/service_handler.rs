//! `DefaultServiceHandler` impl path mirror — SEA Rule 121.
//!
//! This path-level mirror lets the structural auditor match
//! `core/handler/service_handler.rs` to an api counterpart now that
//! `core/handler/` no longer nests a `traits/` subdirectory.

/// SEA Rule 121 marker — path co-location sentinel for
/// `core/handler/service_handler.rs`.
pub(crate) const _RULE_121: () = ();
