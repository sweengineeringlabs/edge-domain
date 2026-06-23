//! Integration tests — `Principal` trait via `AnonymousPrincipal`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::{AnonymousPrincipal, Principal};

/// @covers: Principal::id — returns non-empty string
#[test]
fn test_id_returns_non_empty_string_happy() {
    let p: &dyn Principal = &AnonymousPrincipal;
    assert!(!p.id().is_empty());
}

/// @covers: Principal::id — returns empty string for a minimal stub
#[test]
fn test_id_empty_principal_returns_empty_error() {
    struct EmptyId;
    impl std::fmt::Debug for EmptyId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "EmptyId")
        }
    }
    impl Principal for EmptyId {
        fn id(&self) -> &str { "" }
        fn kind(&self) -> &str { "test" }
    }
    assert_eq!(EmptyId.id(), "");
}

/// @covers: Principal::kind — anonymous kind is stable across calls
#[test]
fn test_kind_is_stable_across_calls_edge() {
    let p = AnonymousPrincipal;
    let kind1 = p.kind();
    let kind2 = p.kind();
    assert_eq!(kind1, kind2, "kind must be stable across calls");
    assert_eq!(kind1, "anonymous", "kind must return expected value");
}

/// @covers: Principal::id — anonymous id matches expected value
#[test]
fn test_id_anonymous_principal_returns_anonymous_happy() {
    assert_eq!(AnonymousPrincipal.id(), "anonymous");
}

/// @covers: Principal::kind — anonymous kind matches expected value
#[test]
fn test_kind_anonymous_principal_returns_anonymous_happy() {
    assert_eq!(AnonymousPrincipal.kind(), "anonymous");
}

/// @covers: Principal — zero-cost marker type
#[test]
fn test_anonymous_principal_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<AnonymousPrincipal>(), 0);
}

/// @covers: Principal::id — id of two anonymous principals are the same
#[test]
fn test_id_two_anonymous_principals_match_edge() {
    assert_eq!(AnonymousPrincipal.id(), AnonymousPrincipal.id());
}

/// @covers: Principal::kind — kind of minimal stub principal is not anonymous
#[test]
fn test_kind_non_anonymous_stub_is_not_anonymous_error() {
    struct ServicePrincipal;
    impl std::fmt::Debug for ServicePrincipal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ServicePrincipal")
        }
    }
    impl Principal for ServicePrincipal {
        fn id(&self) -> &str { "svc-1" }
        fn kind(&self) -> &str { "service" }
    }
    assert_ne!(ServicePrincipal.kind(), "anonymous");
}
