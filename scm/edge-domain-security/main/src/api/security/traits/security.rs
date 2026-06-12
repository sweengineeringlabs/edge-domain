//! [`Security`] — primary security guard contract.

use crate::api::security::errors::SecurityError;
use crate::api::security::types::security_context::SecurityContext;

/// Security guard — verifies that a [`SecurityContext`] satisfies the policy
/// before a handler executes.
///
/// Implement to enforce authentication or authorisation rules.  Use
/// [`NoopSecurity`](crate::NoopSecurity) in tests or for open routes.
pub trait Security: Send + Sync {
    /// Enforce the security policy for the given context.
    ///
    /// Returns [`SecurityError::Unauthenticated`] when authentication is
    /// required but `ctx.authenticated` is `false`.
    fn enforce(&self, ctx: &SecurityContext) -> Result<(), SecurityError>;
}
