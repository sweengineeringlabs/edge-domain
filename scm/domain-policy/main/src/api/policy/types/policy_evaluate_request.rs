//! [`PolicyEvaluateRequest`] — request to evaluate a policy against an input.

/// Request to evaluate a [`Policy`](crate::api::policy::traits::Policy) against `input`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyEvaluateRequest<'a, I> {
    /// The value to evaluate the policy against.
    pub input: &'a I,
}
