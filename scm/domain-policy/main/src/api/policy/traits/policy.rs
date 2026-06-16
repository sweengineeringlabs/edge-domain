//! `Policy` — a named, testable business rule.

use crate::api::policy::errors::PolicyViolation;

/// A named, testable business rule that operates on domain state.
///
/// Distinct from `Validator`, which checks structural correctness — a policy
/// evaluates business intent against current domain state.
///
/// Use [`CompositePolicy`](crate::api::policy::types::composite_policy::CompositePolicy)
/// to combine multiple policies with AND semantics.
pub trait Policy: Send + Sync {
    /// The input type this policy evaluates.
    type Input;

    /// A human-readable name for this policy.
    fn name(&self) -> &'static str;

    /// Evaluate the policy against `input`.
    ///
    /// Returns `Ok(())` if the rule is satisfied, or `Err(PolicyViolation)`.
    fn evaluate(&self, input: &Self::Input) -> Result<(), PolicyViolation>;
}
