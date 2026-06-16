//! `Security` impl for `NoopSecurity`.

use crate::api::security::errors::SecurityError;
use crate::api::security::traits::security::Security;
use crate::api::security::types::noop_security::NoopSecurity;
use crate::api::security::types::security_context::SecurityContext;

impl Security for NoopSecurity {
    fn enforce(&self, _ctx: &SecurityContext) -> Result<(), SecurityError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enforce_authenticated_context_returns_ok() {
        let ctx = SecurityContext::unauthenticated();
        assert!(NoopSecurity.enforce(&ctx).is_ok());
    }
}
