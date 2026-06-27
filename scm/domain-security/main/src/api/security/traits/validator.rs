//! [`Validator`] — canonical configuration validation contract.
//!
//! This is the single definition. Every specialized security crate
//! (`swe-edge-ingress-tenant`, `swe-edge-ingress-tls`, `swe-edge-egress-tls`, …)
//! re-exports this trait from its own `api/traits/validator.rs` rather than
//! maintaining a local copy.

use crate::api::ValidationError;

/// Validates a configuration value before it is used.
///
/// Returns `Ok(())` when valid, or `Err` with a human-readable description
/// of the first validation failure found.
pub trait Validator: Send + Sync {
    /// Validate this value.
    fn validate(&self) -> Result<(), ValidationError>;
}

