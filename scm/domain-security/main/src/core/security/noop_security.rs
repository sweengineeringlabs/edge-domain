//! `Security` impl for `NoopSecurity`.

use crate::api::SecurityError;
use crate::api::Security;
use crate::api::NoopSecurity;
use crate::api::SecurityContext;

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
