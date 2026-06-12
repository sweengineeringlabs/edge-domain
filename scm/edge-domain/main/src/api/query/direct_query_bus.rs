//! `DirectQueryBus` — type alias re-exporting from the designated struct home.
/// Marker for an inline `QueryBus` that dispatches queries in the same task.
pub type DirectQueryBus = crate::api::query::types::DirectQueryBus;
