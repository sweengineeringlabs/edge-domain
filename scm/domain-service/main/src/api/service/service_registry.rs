//! `ServiceRegistry` — api/core mirror for the in-process service registry.
//!
//! This module exists so the structural auditor can match
//! `core/service/service_registry.rs` to an api counterpart at the same path level.
//! The authoritative struct declaration lives in `api/service/types/service_registry.rs`.

use crate::api::service::types;

/// Thread-safe, in-process registry that maps service names to [`Service`](crate::api::service::traits::service::Service) implementations.
///
/// This is a re-export alias so that callers can import from `api::service::ServiceRegistry`
/// without knowing the internal `types/` submodule.
pub type ServiceRegistry<Req, Resp> = types::ServiceRegistry<Req, Resp>;
