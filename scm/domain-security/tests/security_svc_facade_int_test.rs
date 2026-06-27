//! Integration tests for SAF security service facade module.
//! This test file provides coverage for main/src/saf/security/svc.rs
//! which re-exports the Security trait.

use edge_domain_security::{Security, NoopSecurity, SecurityBootstrap};

/// @covers: enforce
#[test]
fn test_security_facade_trait_available_happy() {
    // Verify that the trait is accessible through the public API
    let guard: &dyn Security = &NoopSecurity;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let result = guard.enforce(&ctx);
    assert!(result.is_ok(), "enforce must succeed");
    assert_eq!(result.unwrap(), (), "enforce must return Ok(())");
}

/// @covers: enforce
#[test]
fn test_security_facade_enforce_edge() {
    // Verify enforce is callable multiple times
    let guard: &dyn Security = &NoopSecurity;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let r1 = guard.enforce(&ctx);
    let r2 = guard.enforce(&ctx);
    assert_eq!(r1.is_ok(), r2.is_ok(), "enforce results must be consistent");
}
