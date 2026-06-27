//! [`AuthzPolicy`] — authorization enforcement contract.

use crate::api::security::types::SecurityContext;
use crate::api::SecurityError;

/// Enforces an authorization rule against a [`SecurityContext`].
///
/// Implement on a policy struct to enforce authorization checks.
/// Return `Ok(())` to allow; return [`SecurityError::Auth`] to reject.
pub trait AuthzPolicy: Send + Sync {
    /// Evaluate the policy against `ctx`.
    ///
    /// Returns `Ok(())` if the caller is authorized; `Err(SecurityError::Auth(…))`
    /// otherwise.
    fn check(&self, ctx: &SecurityContext) -> Result<(), SecurityError>;
}

