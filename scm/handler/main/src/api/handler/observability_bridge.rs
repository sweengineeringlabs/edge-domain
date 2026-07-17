//! Blanket-bridge path mirror — SEA Rule 121.
//!
//! This path-level mirror lets the structural auditor match
//! `core/handler/observability_bridge.rs` (the observability trait blanket
//! bridges) to an api counterpart.

/// SEA Rule 121 marker — path co-location sentinel for
/// `core/handler/observability_bridge.rs`.
pub(crate) const _RULE_121: () = ();
