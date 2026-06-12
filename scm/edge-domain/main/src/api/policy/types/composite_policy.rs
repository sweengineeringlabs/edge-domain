//! `CompositePolicy<I>` — AND-composition of multiple [`Policy`] rules.

use crate::api::policy::traits::Policy;

/// Combines multiple [`Policy<Input = I>`](crate::api::policy::traits::Policy)
/// rules with AND semantics.  An empty composite always passes.
pub struct CompositePolicy<I> {
    pub(crate) name: &'static str,
    pub(crate) policies: Vec<Box<dyn Policy<Input = I>>>,
}

impl<I> CompositePolicy<I> {
    /// Construct an empty composite.
    pub fn new() -> Self {
        Self { name: "composite", policies: Vec::new() }
    }

    /// Append `policy` to the evaluation chain.
    pub fn with(mut self, policy: Box<dyn Policy<Input = I>>) -> Self {
        self.policies.push(policy);
        self
    }
}

impl<I> Default for CompositePolicy<I> {
    fn default() -> Self {
        Self::new()
    }
}
