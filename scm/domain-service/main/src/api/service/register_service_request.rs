//! `RegisterServiceRequest` — api/core mirror for the service registration request.
//!
//! This module exists so the structural auditor can match
//! `core/service/register_service_request.rs` to an api counterpart at the same path level.
//! The authoritative struct declaration lives in `api/service/types/register_service_request.rs`.

use crate::api::service::types;

/// Request to register a service in the registry.
///
/// This is a re-export alias so that callers can import from `api::service::RegisterServiceRequest`
/// without knowing the internal `types/` submodule.
pub type RegisterServiceRequest<Req, Resp> = types::RegisterServiceRequest<Req, Resp>;
