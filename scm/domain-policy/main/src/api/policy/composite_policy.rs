//! `CompositePolicy` — SEA Rule 121 api/core mirror.

/// AND-composition of multiple [`Policy`](crate::Policy) rules.
pub type CompositePolicy<I> = crate::api::policy::types::composite_policy::CompositePolicy<I>;
