//! `Policy` — a named, testable business rule.

use crate::api::policy::types::PolicyViolation;

/// A named, testable business rule that operates on domain state.
///
/// Distinct from [`Validator`](crate::api::validator::traits::Validator), which
/// checks structural correctness (non-empty fields, range bounds, etc.) —
/// a policy evaluates *business intent* against current domain state: "can this
/// order be cancelled?", "does this transfer violate a spending limit?".
///
/// # Sync-only
///
/// [`evaluate`](Policy::evaluate) is synchronous — pure business rules should
/// not reach out to infrastructure.  Async side-effects (persisting audit logs,
/// publishing events) are the caller's responsibility after evaluation.
///
/// # Composition
///
/// Use [`crate::api::policy::types::composite_policy::CompositePolicy`] to
/// combine multiple policies with AND semantics — the first violation
/// short-circuits and is returned.
pub trait Policy: Send + Sync {
    /// The input type this policy evaluates.
    type Input;

    /// A human-readable name for this policy.
    ///
    /// Used in audit logs and [`PolicyViolation`] messages.
    /// Must be a `'static` string — typically a module path or kebab-case label.
    fn name(&self) -> &'static str;

    /// Evaluate the policy against `input`.
    ///
    /// Returns `Ok(())` if the rule is satisfied, or `Err(PolicyViolation)` with
    /// the name of the violated rule and a human-readable reason.
    fn evaluate(&self, input: &Self::Input) -> Result<(), PolicyViolation>;
}
