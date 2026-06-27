//! Integration tests for [`AuthzPolicy`] trait.

use edge_domain_security::{AuthzPolicy, SecurityContext, SecurityError};

struct AllowPolicy;
impl AuthzPolicy for AllowPolicy {
    fn check(&self, _ctx: &SecurityContext) -> Result<(), SecurityError> {
        Ok(())
    }
}

struct DenyPolicy;
impl AuthzPolicy for DenyPolicy {
    fn check(&self, _ctx: &SecurityContext) -> Result<(), SecurityError> {
        Err(SecurityError::Auth("denied".to_string()))
    }
}

/// @covers: check
#[test]
fn test_authz_policy_check_happy() {
    let policy = AllowPolicy;
    let ctx = SecurityContext::unauthenticated();
    assert!(policy.check(&ctx).is_ok());
}

/// @covers: check
#[test]
fn test_authz_policy_check_error() {
    let policy = DenyPolicy;
    let ctx = SecurityContext::unauthenticated();
    assert!(policy.check(&ctx).is_err());
}

/// @covers: check
#[test]
fn test_authz_policy_check_edge() {
    let policy = AllowPolicy;
    let ctx = SecurityContext::unauthenticated();
    let r1 = policy.check(&ctx);
    let r2 = policy.check(&ctx);
    assert_eq!(r1.is_ok(), r2.is_ok());
}
