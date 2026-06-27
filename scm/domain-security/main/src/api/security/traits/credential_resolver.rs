//! [`CredentialResolver`] — outbound credential resolution and inbound token verification.

use crate::api::security::types::SecurityContext;
use crate::api::Claims;
use crate::api::CredentialSource;
use crate::api::SecretString;
use crate::api::SecurityError;
use crate::api::Token;

/// Verifies inbound bearer tokens and resolves outbound credentials.
///
/// Infrastructure crates implement this for JWT/TLS/OIDC backends and secret
/// stores. Use [`crate::NoopCredentialResolver`] in tests or for open routes.
pub trait CredentialResolver: Send + Sync {
    /// Verify a bearer token and extract its JWT claims.
    ///
    /// Returns [`SecurityError::Token`] when the token is invalid or expired.
    fn verify(&self, token: &Token) -> Result<Claims, SecurityError>;

    /// Fetch the outbound secret for the named credential source.
    ///
    /// `source` identifies the target service (e.g. `"payment-gateway"`).
    /// `ctx` scopes the lookup to the current caller's security context.
    ///
    /// Returns [`SecurityError::Auth`] when the credential cannot be resolved.
    fn resolve(
        &self,
        source: &CredentialSource,
        ctx: &SecurityContext,
    ) -> Result<SecretString, SecurityError>;
}
