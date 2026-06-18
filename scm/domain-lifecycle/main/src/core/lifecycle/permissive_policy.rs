//! `TransitionPolicy` impl for `PermissivePolicy`.

use std::fmt::Debug;

use crate::api::PermissivePolicy;
use crate::api::TransitionPolicy;

impl<S: Copy + Eq + Debug + Send + Sync + 'static> TransitionPolicy for PermissivePolicy<S> {
    type State = S;

    fn is_allowed(&self, _from: S, _to: S) -> bool {
        true
    }
}
