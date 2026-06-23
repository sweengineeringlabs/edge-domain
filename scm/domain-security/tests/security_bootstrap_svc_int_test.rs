//! Integration tests — `SecurityBootstrap` SAF facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use edge_domain_security::{
    AnonymousPrincipal, NoopSecurity, Security, SecurityContext, SecurityContextBuilder,
    SecurityError, SecurityBootstrap, SecurityServices,
};

struct TestSecurity;
impl SecurityBootstrap for TestSecurity {}

/// @covers: SecurityBootstrap::unauthenticated — returns unauthenticated context
#[test]
fn test_unauthenticated_returns_false_authenticated_flag_happy() {
    let ctx: SecurityContext = TestSecurity::unauthenticated();
    assert!(!ctx.authenticated);
}

/// @covers: SecurityBootstrap::authenticated — sets authenticated flag
#[test]
fn test_authenticated_sets_true_flag_happy() {
    let ctx = TestSecurity::authenticated(Box::new(AnonymousPrincipal));
    assert!(ctx.authenticated);
}

/// @covers: SecurityBootstrap::unauthenticated — principal is None
#[test]
fn test_unauthenticated_principal_is_none_error() {
    let ctx = TestSecurity::unauthenticated();
    assert!(ctx.principal.is_none());
}

/// @covers: SecurityBootstrap::authenticated — principal id is preserved
#[test]
fn test_authenticated_principal_id_is_preserved_error() {
    let ctx = TestSecurity::authenticated(Box::new(AnonymousPrincipal));
    let id = ctx.principal.as_ref().map(|p| p.id()).unwrap_or("");
    assert_eq!(id, AnonymousPrincipal::ID);
}

/// @covers: SecurityBootstrap::authenticated — claims are empty by default
#[test]
fn test_authenticated_claims_empty_by_default_edge() {
    let ctx = TestSecurity::authenticated(Box::new(AnonymousPrincipal));
    assert!(ctx.claims.is_empty());
}

/// @covers: SecurityBootstrap::from_claims — non-empty map succeeds
#[test]
fn test_from_claims_nonempty_map_returns_ok_happy() {
    let mut claims = HashMap::new();
    claims.insert("sub".to_string(), "user-1".to_string());
    let result = TestSecurity::from_claims(claims);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().claim("sub"), Some("user-1"));
}

/// @covers: SecurityBootstrap::from_claims — empty map returns MissingClaims
#[test]
fn test_from_claims_empty_map_returns_missing_claims_error() {
    let err = TestSecurity::from_claims(HashMap::new()).unwrap_err();
    assert_eq!(err, SecurityError::MissingClaims);
}

/// @covers: SecurityBootstrap::from_claims — multiple claims all stored
#[test]
fn test_from_claims_multiple_entries_all_stored_edge() {
    let claims = [
        ("role".to_string(), "admin".to_string()),
        ("tenant".to_string(), "acme".to_string()),
    ]
    .into_iter()
    .collect();
    let ctx = TestSecurity::from_claims(claims).unwrap();
    assert_eq!(ctx.claim("role"), Some("admin"));
    assert_eq!(ctx.claim("tenant"), Some("acme"));
}

/// @covers: SecurityBootstrap::noop_guard — returns NoopSecurity
#[test]
fn test_noop_guard_constructs_successfully_happy() {
    let guard: NoopSecurity = TestSecurity::noop_guard();
    let ctx = TestSecurity::unauthenticated();
    assert_eq!(guard.enforce(&ctx), Ok(()));
}

/// @covers: SecurityBootstrap::noop_guard — is zero-sized
#[test]
fn test_noop_guard_is_zero_sized_error() {
    assert_eq!(std::mem::size_of_val(&TestSecurity::noop_guard()), 0);
}

/// @covers: SecurityBootstrap::noop_guard — successive calls are independent
#[test]
fn test_noop_guard_successive_calls_are_independent_edge() {
    let _a = TestSecurity::noop_guard();
    let _b = TestSecurity::noop_guard();
    assert_eq!(_a, _b);
}

/// @covers: SecurityBootstrap::anonymous_principal — returns AnonymousPrincipal
#[test]
fn test_anonymous_principal_constructs_successfully_happy() {
    let ap: AnonymousPrincipal = TestSecurity::anonymous_principal();
    assert_eq!(std::mem::size_of_val(&ap), 0);
}

/// @covers: SecurityBootstrap::anonymous_principal — is zero-sized
#[test]
fn test_anonymous_principal_is_zero_sized_error() {
    assert_eq!(std::mem::size_of_val(&TestSecurity::anonymous_principal()), 0);
}

/// @covers: SecurityBootstrap::anonymous_principal — successive calls equal
#[test]
fn test_anonymous_principal_successive_calls_equal_edge() {
    assert_eq!(TestSecurity::anonymous_principal(), AnonymousPrincipal);
}

/// @covers: SecurityServices — constructs without arguments (zero-config bootstrap)
#[test]
fn test_security_services_constructs_without_args_happy() {
    let guard = SecurityServices::noop_guard();
    let ctx = SecurityServices::unauthenticated();
    assert_eq!(guard.enforce(&ctx), Ok(()));
}

/// @covers: SecurityServices — is zero-sized
#[test]
fn test_security_services_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<SecurityServices>(), 0);
}

/// @covers: SecurityServices — bootstrap methods match TestSecurity bootstrap methods
#[test]
fn test_security_services_anonymous_principal_matches_test_factory_edge() {
    assert_eq!(SecurityServices::anonymous_principal(), TestSecurity::anonymous_principal());
}

/// @covers: SecurityBootstrap::context_builder — returns a usable builder
#[test]
fn test_context_builder_returns_usable_builder_happy() {
    let ctx: edge_domain_security::SecurityContext = TestSecurity::context_builder().build();
    assert!(!ctx.authenticated);
}

/// @covers: SecurityBootstrap::context_builder — builder starts with empty claims
#[test]
fn test_context_builder_starts_with_empty_claims_error() {
    let ctx = TestSecurity::context_builder().build();
    assert!(ctx.claims.is_empty());
}

/// @covers: SecurityBootstrap::context_builder — successive calls are independent
#[test]
fn test_context_builder_successive_calls_independent_edge() {
    let a = TestSecurity::context_builder().claim("k", "1").build();
    let b = TestSecurity::context_builder().claim("k", "2").build();
    assert_ne!(a.claim("k"), b.claim("k"));
}

/// @covers: SecurityBootstrap::default_services — returns SecurityServices
#[test]
fn test_default_services_constructs_successfully_happy() {
    let ss: SecurityServices = TestSecurity::default_services();
    assert_eq!(std::mem::size_of_val(&ss), 0);
}

/// @covers: SecurityBootstrap::default_services — is zero-sized
#[test]
fn test_default_services_is_zero_sized_error() {
    assert_eq!(std::mem::size_of_val(&TestSecurity::default_services()), 0);
}

/// @covers: SecurityBootstrap::default_services — successive calls equal
#[test]
fn test_default_services_successive_calls_equal_edge() {
    assert_eq!(TestSecurity::default_services(), SecurityServices);
}

/// @covers: SecurityContextBuilder — used via bootstrap method
#[test]
fn test_security_context_builder_via_factory_happy() {
    let ctx: SecurityContext = TestSecurity::context_builder()
        .principal(Box::new(TestSecurity::anonymous_principal()))
        .build();
    assert!(ctx.authenticated);
}

/// @covers: SecurityContextBuilder — type is accessible from bootstrap
#[test]
fn test_security_context_builder_type_accessible_error() {
    let builder = SecurityContextBuilder::new();
    let ctx = builder.build();
    assert!(!ctx.authenticated);
}

/// @covers: SecurityContextBuilder — default bootstrap and new() produce equal initial state
#[test]
fn test_security_context_builder_default_and_new_equal_edge() {
    let a = TestSecurity::context_builder().build();
    let b = SecurityContextBuilder::new().build();
    assert_eq!(a.authenticated, b.authenticated);
    assert!(a.principal.is_none());
}
