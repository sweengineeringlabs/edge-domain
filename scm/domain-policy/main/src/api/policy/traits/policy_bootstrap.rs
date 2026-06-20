//! [`PolicyBootstrap`] — constructor contract for policy infrastructure types.

use crate::api::policy::types::{CompositePolicy, StdPolicyFactory};

/// Bootstrap trait for the standard policy composition types.
pub trait PolicyBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "policy"
    }

    /// Construct an empty `CompositePolicy` with AND-composition semantics.
    fn composite<I>() -> CompositePolicy<I> where Self: Sized {
        CompositePolicy::new()
    }

    /// Return the standard policy-factory instance.
    fn std_factory() -> StdPolicyFactory where Self: Sized {
        StdPolicyFactory
    }
}
