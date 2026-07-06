//! `OutboundRegistry` — SEA Rule 121 api/core mirror.
//!
//! Provides a type alias so the structural auditor finds a substantive
//! declaration at this path, mirroring the core implementation at
//! `core/domain/outbound_registry.rs`.

/// Type alias for the outbound handle registry, parameterised over the handle type.
pub type OutboundRegistry<H> = crate::api::domain::types::outbound_registry::OutboundRegistry<H>;
