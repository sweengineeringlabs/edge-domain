//! [`PolicyFactory`] — constructor contract for policy infrastructure types.

use crate::api::policy::types::{CompositePolicy, StdPolicyFactory};

/// Factory trait for the standard policy composition types.
pub trait PolicyFactory {
    /// Construct an empty `CompositePolicy` with AND-composition semantics.
    fn composite<I>() -> CompositePolicy<I> {
        CompositePolicy::new()
    }

    /// Return the standard policy-factory instance.
    fn std_factory() -> StdPolicyFactory {
        StdPolicyFactory
    }
}
