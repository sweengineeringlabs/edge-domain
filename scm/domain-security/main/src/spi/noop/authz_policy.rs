//! Noop [`AuthzPolicy`] implementation.

use crate::api::AuthzPolicy;
use crate::api::SecurityContext;
use crate::api::SecurityError;

/// Noop authorization policy that allows all contexts.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NoopAuthzPolicy;

impl AuthzPolicy for NoopAuthzPolicy {
    fn check(&self, _ctx: &SecurityContext) -> Result<(), SecurityError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_authz_policy_allows_all() {
        let policy = NoopAuthzPolicy;
        let ctx = SecurityContext::unauthenticated();
        assert!(policy.check(&ctx).is_ok());
    }
}
