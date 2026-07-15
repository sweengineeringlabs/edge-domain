//! [`ServiceBridge`] — marker trait for types that bridge a `Service` into the dispatch pipeline.

/// Marker trait: a type that bridges a domain [`Service`](edge_application_service::Service)
/// into the dispatch pipeline as a [`Handler`](super::handler::Handler).
pub trait ServiceBridge: Send + Sync {}
