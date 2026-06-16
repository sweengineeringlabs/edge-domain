//! `InProcessHandlerRegistry` — SEA Rule 121 api/core mirror.
//!
//! Provides a type alias so the structural auditor finds a substantive
//! declaration at this path, mirroring the core implementation at
//! `core/handler/in_process_handler_registry.rs`.

/// Type alias for the in-process handler registry, parameterised over request and response types.
///
/// Prefer this alias over naming `types::InProcessHandlerRegistry` directly in call sites.
pub type InProcessHandlerRegistry<Req, Resp> =
    crate::api::handler::types::in_process_handler_registry::InProcessHandlerRegistry<Req, Resp>;
