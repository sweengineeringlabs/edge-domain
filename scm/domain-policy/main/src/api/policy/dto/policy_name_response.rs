//! [`PolicyNameResponse`] — wrapper for a policy's human-readable name.

/// Result of [`Policy::name`](crate::api::policy::traits::Policy::name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyNameResponse {
    /// A human-readable name for this policy.
    pub name: &'static str,
}
