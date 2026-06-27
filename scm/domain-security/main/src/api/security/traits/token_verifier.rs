//! [`TokenVerifier`] — verify inbound tokens without direction bias.

use crate::api::Claims;
use crate::api::SecurityError;

/// Verifies opaque bearer tokens and extracts standard JWT claims.
///
/// Trait with no directional bias — implemented on both ingress (HTTP/gRPC verifiers)
/// and egress (callback verification) sides. Test: "could this be implemented on both
/// sides of the boundary?" Yes → belongs in shared.
pub trait TokenVerifier: Send + Sync {
    /// Verify the given token string and extract standard JWT claims.
    ///
    /// Returns `Claims` (all fields optional) for successfully verified tokens.
    /// Returns `SecurityError::Verification` or other variant on failure.
    fn verify(&self, token: &str) -> Result<Claims, SecurityError>;
}

