//! `LifecycleFactory` — constructor contract for lifecycle implementations.

use std::fmt::Debug;

use crate::api::lifecycle::traits::TransitionPolicy;
use crate::api::lifecycle::types::{ManagedLifecycle, PermissivePolicy, StdLifecycleFactory};

/// Factory trait for the standard `Lifecycle` implementation.
pub trait LifecycleFactory {
    /// Construct a [`ManagedLifecycle`] in `initial` state, governed by `policy`.
    fn managed<S: Copy + Eq + Debug + Send + Sync + 'static>(
        initial: S,
        policy: Box<dyn TransitionPolicy<State = S>>,
    ) -> ManagedLifecycle<S> {
        ManagedLifecycle::new(initial, policy)
    }

    /// Construct a [`ManagedLifecycle`] in `initial` state with a
    /// [`PermissivePolicy`] — every transition is allowed.
    fn permissive<S: Copy + Eq + Debug + Send + Sync + 'static>(initial: S) -> ManagedLifecycle<S> {
        ManagedLifecycle::new(initial, Box::new(PermissivePolicy::new()))
    }

    /// Return the standard lifecycle-factory instance.
    fn std_factory() -> StdLifecycleFactory {
        StdLifecycleFactory
    }
}
