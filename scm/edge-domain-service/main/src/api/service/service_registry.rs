//! `ServiceRegistry` — SEA Rule 121 api/core mirror.
//!
//! `ServiceRegistry` is the api-layer struct type for the in-process service
//! registry. This path-level mirror lets the structural auditor match
//! `core/service/service_registry.rs` to an api counterpart.
pub use crate::api::service::types::ServiceRegistry;
