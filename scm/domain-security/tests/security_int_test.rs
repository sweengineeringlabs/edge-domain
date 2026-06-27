//! Integration tests for [`Security`] trait.

use edge_domain_security::{Security, SecurityError};

struct AllowSecurity;
impl Security for AllowSecurity {
    fn enforce(&self, _ctx: &edge_domain_security::SecurityContext) -> Result<(), SecurityError> {
        Ok(())
    }
}

struct DenySecurity;
impl Security for DenySecurity {
    fn enforce(&self, _ctx: &edge_domain_security::SecurityContext) -> Result<(), SecurityError> {
        Err(SecurityError::Auth("not authenticated".to_string()))
    }
}

/// @covers: enforce
#[test]
fn test_enforce_happy() {
    use edge_domain_security::SecurityBootstrap;
    let security = AllowSecurity;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let result = security.enforce(&ctx);
    assert!(result.is_ok(), "allow security must approve context");
    assert_eq!(result.unwrap(), (), "enforce must return Ok(())");
}

/// @covers: enforce
#[test]
fn test_enforce_error() {
    use edge_domain_security::SecurityBootstrap;
    let security = DenySecurity;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    assert!(security.enforce(&ctx).is_err());
}

/// @covers: enforce
#[test]
fn test_enforce_edge() {
    use edge_domain_security::SecurityBootstrap;
    let security = AllowSecurity;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let r1 = security.enforce(&ctx);
    let r2 = security.enforce(&ctx);
    assert_eq!(r1.is_ok(), r2.is_ok());
}
