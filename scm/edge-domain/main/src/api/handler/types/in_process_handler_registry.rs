//! `InProcessHandlerRegistry` — marker type for the in-process handler registry.

/// In-process handler registry backed by `parking_lot::RwLock<HashMap>`.
///
/// Access via [`Domain::new_handler_registry`](crate::Domain::new_handler_registry).
pub struct InProcessHandlerRegistry;
