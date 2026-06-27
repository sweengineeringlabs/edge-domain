//! [`Claims`] inherent impls.

use crate::Claims;

impl Claims {
    /// Returns `true` if `exp` is set and the token has not yet expired
    /// relative to the given Unix timestamp.
    pub(crate) fn is_valid_at(&self, now_secs: u64) -> bool {
        self.exp.is_none_or(|exp| now_secs < exp)
    }
}
