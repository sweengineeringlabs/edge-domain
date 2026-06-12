//! [`PolicyFactory`] — constructor contract for policy infrastructure types.

use crate::api::policy::types::composite_policy::CompositePolicy;

/// Factory trait for the standard policy composition types.
pub trait PolicyFactory {
    /// Construct an empty [`CompositePolicy`] with AND-composition semantics.
    fn composite<I>() -> CompositePolicy<I> {
        CompositePolicy::new()
    }
}
