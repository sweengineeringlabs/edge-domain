//! [`Claims`] inherent impls.

use crate::Claims;

impl Claims {
    /// Returns `true` if `exp` is set and the token has not yet expired
    /// relative to the given Unix timestamp.
    pub(crate) fn is_valid_at(&self, now_secs: u64) -> bool {
        self.exp.is_none_or(|exp| now_secs < exp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_at_no_expiry() {
        let claims = Claims::default();
        assert!(claims.is_valid_at(1000));
    }

    #[test]
    fn test_is_valid_at_not_expired() {
        let mut claims = Claims::default();
        claims.exp = Some(2000);
        assert!(claims.is_valid_at(1000));
    }

    #[test]
    fn test_is_valid_at_expired() {
        let mut claims = Claims::default();
        claims.exp = Some(1000);
        assert!(!claims.is_valid_at(2000));
    }
}
