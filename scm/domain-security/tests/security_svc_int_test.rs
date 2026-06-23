//! Integration tests — `Security` trait via SAF re-export.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::{AnonymousPrincipal, NoopSecurity, Security, SecurityContext};

/// @covers: Security::enforce — noop guard allows any context
#[test]
fn test_enforce_noop_guard_allows_any_context_happy() {
    let guard: &dyn Security = &NoopSecurity;
    let ctx = SecurityContext::unauthenticated();
    assert_eq!(guard.enforce(&ctx), Ok(()));
}

/// @covers: Security::enforce — strict guard rejects unauthenticated
#[test]
fn test_enforce_strict_guard_rejects_unauthenticated_error() {
    struct StrictSecurity;
    impl Security for StrictSecurity {
        fn enforce(&self, ctx: &SecurityContext) -> Result<(), edge_domain_security::SecurityError> {
            if ctx.authenticated {
                Ok(())
            } else {
                Err(edge_domain_security::SecurityError::Unauthenticated)
            }
        }
    }
    let guard = StrictSecurity;
    let ctx = SecurityContext::unauthenticated();
    assert_eq!(guard.enforce(&ctx), Err(edge_domain_security::SecurityError::Unauthenticated));
}

/// @covers: Security::enforce — guard is object-safe (dyn dispatch works)
#[test]
fn test_enforce_guard_is_object_safe_edge() {
    let guards: Vec<Box<dyn Security>> = vec![Box::new(NoopSecurity)];
    let ctx = SecurityContext::authenticated_with(Box::new(AnonymousPrincipal));
    for g in &guards {
        assert_eq!(g.enforce(&ctx), Ok(()));
    }
}
