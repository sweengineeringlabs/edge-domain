//! `TransitionPolicy` impl for `PermissivePolicy`.

use std::fmt::Debug;

use crate::api::lifecycle::permissive_policy::PermissivePolicy;
use crate::api::lifecycle::traits::TransitionPolicy;

impl<S: Copy + Eq + Debug + Send + Sync + 'static> TransitionPolicy for PermissivePolicy<S> {
    type State = S;

    fn is_allowed(&self, _from: S, _to: S) -> bool {
        true
    }
}
