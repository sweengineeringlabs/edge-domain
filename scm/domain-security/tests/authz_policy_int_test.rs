//! Integration tests for [`AuthzPolicy`] trait.

use edge_domain_security::{AuthzPolicy, SecurityError, SecurityBootstrap};

struct AllowPolicy;
impl AuthzPolicy for AllowPolicy {
    fn check(&self, _ctx: &edge_domain_security::SecurityContext) -> Result<(), SecurityError> {
        Ok(())
    }
}

struct DenyPolicy;
impl AuthzPolicy for DenyPolicy {
    fn check(&self, _ctx: &edge_domain_security::SecurityContext) -> Result<(), SecurityError> {
        Err(SecurityError::Auth("denied".to_string()))
    }
}

/// @covers: AuthzPolicy::check
#[test]
fn test_check_allow_happy() {
    let policy = AllowPolicy;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let result = policy.check(&ctx);
    assert!(result.is_ok(), "allow policy must approve unauthenticated context");
    assert_eq!(result.unwrap(), (), "allow policy must return Ok(())");
}

/// @covers: AuthzPolicy::check
#[test]
fn test_check_deny_error() {
    let policy = DenyPolicy;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    assert!(policy.check(&ctx).is_err());
}

/// @covers: AuthzPolicy::check
#[test]
fn test_check_consistent_edge() {
    let policy = AllowPolicy;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let r1 = policy.check(&ctx);
    let r2 = policy.check(&ctx);
    assert_eq!(r1.is_ok(), r2.is_ok());
}
