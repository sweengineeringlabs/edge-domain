//! `CompositePolicy` — type alias re-exporting from the designated struct home.
/// AND-composition of multiple [`Policy`](crate::api::policy::traits::Policy) rules.
pub type CompositePolicy<I> = crate::api::policy::types::CompositePolicy<I>;
