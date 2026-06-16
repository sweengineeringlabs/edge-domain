//! `PermissivePolicy` — transition policy that allows every transition.

use std::marker::PhantomData;

/// A [`TransitionPolicy`](crate::api::lifecycle::traits::TransitionPolicy) that
/// permits every state transition unconditionally.
///
/// Useful for tests and for entities whose transitions are externally governed
/// (e.g. by an event sourcing aggregate that replays recorded events).
pub struct PermissivePolicy<S>(PhantomData<fn(S)>);

impl<S> PermissivePolicy<S> {
    /// Construct a new permissive policy.
    pub fn new() -> Self {
        PermissivePolicy(PhantomData)
    }
}

impl<S> Default for PermissivePolicy<S> {
    fn default() -> Self {
        Self::new()
    }
}
