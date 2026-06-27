//! Integration tests for [`NoopSecurity`] type.

use edge_domain_security::{NoopSecurity, Security, SecurityBootstrap};

#[test]
fn test_noop_security_enforce_happy() {
    let guard = NoopSecurity;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let result = guard.enforce(&ctx);
    assert!(result.is_ok(), "NoopSecurity must allow all contexts");
    assert_eq!(result.unwrap(), (), "enforce must return Ok(())");
}

#[test]
fn test_noop_security_enforce_error() {
    let guard = NoopSecurity;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let result = guard.enforce(&ctx);
    assert!(result.is_ok(), "NoopSecurity must not reject any context");
    assert_eq!(result.unwrap(), (), "enforce must return Ok(())");
}

#[test]
fn test_noop_security_enforce_edge() {
    let guard = NoopSecurity;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let r1 = guard.enforce(&ctx);
    let r2 = guard.enforce(&ctx);
    assert_eq!(r1.is_ok(), r2.is_ok(), "NoopSecurity must be consistent");
}

#[test]
fn test_noop_security_zst() {
    assert_eq!(std::mem::size_of::<NoopSecurity>(), 0, "NoopSecurity must be zero-sized");
}

#[test]
fn test_noop_security_copy_happy() {
    let _guard: NoopSecurity = NoopSecurity;
    let guard_copy = _guard;
    assert!(std::mem::size_of_val(&guard_copy) == 0, "Copy must preserve zero-size");
}

#[test]
fn test_noop_security_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {
        // Type parameter T is Send + Sync
    }
    // This should compile without error, proving Send + Sync
    assert_send_sync::<NoopSecurity>();
    // Verify the type is still zero-sized after the bound check
    assert_eq!(std::mem::size_of::<NoopSecurity>(), 0);
}
