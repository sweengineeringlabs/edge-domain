//! [`TransitionAllowedResponse`] — wrapper for a transition-permission check.

/// Result of [`TransitionPolicy::is_allowed`](crate::api::lifecycle::traits::TransitionPolicy::is_allowed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionAllowedResponse {
    /// `true` when the transition is permitted.
    pub allowed: bool,
}
