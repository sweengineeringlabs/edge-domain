//! `CompositePolicy<I>` — AND-composition of multiple [`Policy`] rules.

use crate::api::policy::traits::Policy;

/// Combines multiple [`Policy<Input = I>`](crate::api::policy::traits::Policy)
/// rules with AND semantics.
///
/// Each policy is evaluated in registration order. The first
/// [`PolicyViolation`](crate::api::policy::types::PolicyViolation) encountered
/// short-circuits evaluation and is returned. An empty composite always passes.
///
/// # Examples
///
/// ```rust
/// use edge_domain_policy::{CompositePolicy, Policy, PolicyViolation};
///
/// struct MaxLength(usize);
///
/// impl Policy for MaxLength {
///     type Input = String;
///     fn name(&self) -> &'static str { "max-length" }
///     fn evaluate(&self, input: &String) -> Result<(), PolicyViolation> {
///         if input.len() <= self.0 { Ok(()) }
///         else { Err(PolicyViolation::new("max-length", format!("exceeds {}", self.0))) }
///     }
/// }
///
/// let policy = CompositePolicy::new()
///     .with(Box::new(MaxLength(10)));
///
/// assert!(policy.evaluate(&"short".to_string()).is_ok());
/// assert!(policy.evaluate(&"a string that is too long".to_string()).is_err());
/// ```
pub struct CompositePolicy<I> {
    pub(crate) name: &'static str,
    pub(crate) policies: Vec<Box<dyn Policy<Input = I>>>,
}

impl<I> CompositePolicy<I> {
    /// Construct an empty composite. An empty composite always passes.
    pub fn new() -> Self {
        Self {
            name: "composite",
            policies: Vec::new(),
        }
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
