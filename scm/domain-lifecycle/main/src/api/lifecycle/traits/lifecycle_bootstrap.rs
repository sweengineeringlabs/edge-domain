//! `LifecycleBootstrap` — constructor contract for lifecycle implementations.

use std::fmt::Debug;

use crate::api::lifecycle::traits::TransitionPolicy;
use crate::api::lifecycle::types::{ManagedLifecycle, PermissivePolicy, StdLifecycleFactory};

/// Bootstrap trait for the standard `Lifecycle` implementation.
pub trait LifecycleBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "lifecycle"
    }

    /// Construct a [`ManagedLifecycle`] in `initial` state, governed by `policy`.
    fn managed<S: Copy + Eq + Debug + Send + Sync + 'static>(
        initial: S,
        policy: Box<dyn TransitionPolicy<State = S>>,
    ) -> ManagedLifecycle<S>
    where
        Self: Sized,
    {
        ManagedLifecycle::new(initial, policy)
    }

    /// Construct a [`ManagedLifecycle`] in `initial` state with a
    /// [`PermissivePolicy`] — every transition is allowed.
    fn permissive<S: Copy + Eq + Debug + Send + Sync + 'static>(initial: S) -> ManagedLifecycle<S>
    where
        Self: Sized,
    {
        ManagedLifecycle::new(initial, Box::new(PermissivePolicy::new()))
    }

    /// Return the standard lifecycle-factory instance.
    fn std_factory() -> StdLifecycleFactory
    where
        Self: Sized,
    {
        StdLifecycleFactory
    }
}
