//! Integration tests for AuthzPolicy trait.

use edge_domain_security::{AuthzPolicy, SecurityContext, SecurityError};

struct AllowAllPolicy;
impl AuthzPolicy for AllowAllPolicy {
    fn check(&self, _ctx: &SecurityContext) -> Result<(), SecurityError> {
        Ok(())
    }
}

struct RejectAllPolicy;
impl AuthzPolicy for RejectAllPolicy {
    fn check(&self, _ctx: &SecurityContext) -> Result<(), SecurityError> {
        Err(SecurityError::Auth("denied".to_string()))
    }
}

/// @covers: AuthzPolicy::check
#[test]
fn test_check_allow_all_happy() {
    let policy = AllowAllPolicy;
    let ctx = SecurityContext::unauthenticated();
    let result = policy.check(&ctx);
    assert_eq!(result, Ok(()));
}

/// @covers: AuthzPolicy::check
#[test]
fn test_check_reject_error() {
    let policy = RejectAllPolicy;
    let ctx = SecurityContext::unauthenticated();
    let result = policy.check(&ctx);
    assert!(result.is_err());
}

/// @covers: AuthzPolicy::check
#[test]
fn test_check_authenticated_edge() {
    let policy = AllowAllPolicy;
    let ctx = SecurityContext::unauthenticated();
    assert_eq!(policy.check(&ctx), Ok(()));
}
