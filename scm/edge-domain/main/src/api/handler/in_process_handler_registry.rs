//! `InProcessHandlerRegistry` — type alias re-exporting from the designated struct home.
/// Marker for the in-process handler registry backed by `parking_lot::RwLock<HashMap>`.
pub type InProcessHandlerRegistry = crate::api::handler::types::InProcessHandlerRegistry;
