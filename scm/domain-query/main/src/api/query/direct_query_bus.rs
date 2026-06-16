//! `DirectQueryBus` — SEA Rule 121 api/core mirror.
//!
//! This path-level mirror lets the structural auditor match
//! `core/query/direct_query_bus.rs` to an api counterpart.

/// Type alias for the direct (inline) query bus parameterised by result type `R`.
///
/// This is identical to [`crate::api::query::types::DirectQueryBus`] and is
/// provided as a convenience for callers who want to spell the type without
/// navigating the `types/` submodule path.
pub type DirectQueryBus<R> = crate::api::query::types::DirectQueryBus<R>;
