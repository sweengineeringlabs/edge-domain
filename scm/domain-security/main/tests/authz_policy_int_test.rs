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

#[test]
fn test_authz_policy_check_happy() {
    let policy = AllowAllPolicy;
    let ctx = SecurityContext::unauthenticated();
    let result = policy.check(&ctx);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_authz_policy_check_error() {
    let policy = RejectAllPolicy;
    let ctx = SecurityContext::unauthenticated();
    let result = policy.check(&ctx);
    assert!(result.is_err());
}

#[test]
fn test_authz_policy_check_edge_authenticated() {
    let policy = AllowAllPolicy;
    let ctx = SecurityContext::authenticated_with("user-123".to_string());
    assert_eq!(policy.check(&ctx), Ok(()));
}
