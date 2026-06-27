//! `Security` impl for `NoopSecurity`.

use crate::api::NoopSecurity;
use crate::api::Security;
use crate::api::SecurityContext;
use crate::api::SecurityError;

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
        assert_eq!(
            NoopSecurity.enforce(&ctx),
            Ok(()),
            "noop security enforce should return Ok(())"
        );
    }
}
