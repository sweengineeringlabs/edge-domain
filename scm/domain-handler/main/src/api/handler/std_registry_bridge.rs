//! `StdRegistryBridge` — SEA Rule 121 api/core mirror.
//!
//! Provides a type alias so the structural auditor finds a substantive
//! declaration at this path, mirroring the core implementation at
//! `core/handler/std_registry_bridge.rs`. The [`RegistryBridge`](crate::api::handler::traits::RegistryBridge)
//! trait itself is declared at `api/handler/traits/registry_bridge.rs`.

/// Type alias for the standard [`RegistryBridge`](crate::api::handler::traits::RegistryBridge)
/// implementation. Prefer this alias over naming `types::std_registry_bridge::StdRegistryBridge`
/// directly in call sites.
pub type StdRegistryBridge = crate::api::handler::types::std_registry_bridge::StdRegistryBridge;
