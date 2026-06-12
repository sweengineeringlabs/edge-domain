//! `PolicyViolation` — describes why a [`Policy`](crate::api::policy::traits::Policy) was violated.

use thiserror::Error;

/// Returned by [`Policy::evaluate`](crate::api::policy::traits::Policy::evaluate)
/// when a business rule is not satisfied.
///
/// Carries the policy name (for audit logs and routing) and a human-readable
/// reason (for operator messages and structured error responses).
///
/// # Examples
///
/// ```rust
/// use edge_domain::PolicyViolation;
///
/// let v = PolicyViolation::new("spending-limit", "transfer of 500 exceeds daily limit of 200");
/// assert_eq!(v.policy, "spending-limit");
/// assert!(v.reason.contains("500"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("policy '{policy}' violated: {reason}")]
pub struct PolicyViolation {
    /// The [`Policy::name`](crate::api::policy::traits::Policy::name) of the rule that failed.
    pub policy: &'static str,

    /// Human-readable explanation of why the rule was not satisfied.
    pub reason: String,
}

impl PolicyViolation {
    /// Construct a `PolicyViolation` with the given policy name and reason.
    pub fn new(policy: &'static str, reason: impl Into<String>) -> Self {
        Self {
            policy,
            reason: reason.into(),
        }
    }
}
