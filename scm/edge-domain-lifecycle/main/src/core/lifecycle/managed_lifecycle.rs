//! `Lifecycle` impl for `ManagedLifecycle`.

use std::fmt::Debug;

use async_trait::async_trait;

use crate::api::lifecycle::errors::LifecycleError;
use crate::api::lifecycle::managed_lifecycle::ManagedLifecycle;
use crate::api::lifecycle::traits::Lifecycle;

#[async_trait]
impl<S: Copy + Eq + Debug + Send + Sync + 'static> Lifecycle for ManagedLifecycle<S> {
    type State = S;

    fn state(&self) -> S {
        *self.state.read().unwrap_or_else(|p| p.into_inner())
    }

    async fn transition_to(&self, target: S) -> Result<(), LifecycleError> {
        let from = *self.state.read().unwrap_or_else(|p| p.into_inner());
        if !self.policy.is_allowed(from, target) {
            return Err(LifecycleError::InvalidTransition {
                from: format!("{from:?}"),
                to: format!("{target:?}"),
            });
        }
        *self.state.write().unwrap_or_else(|p| p.into_inner()) = target;
        Ok(())
    }
}
