//! Integration tests — `SecurityContext` → `SecurityPrincipal` bridge.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::DirectCommandBus;
use edge_domain_handler::{HandlerContext, ObserverContextAdapter, SecurityPrincipal};
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;

fn assert_security_principal<T: SecurityPrincipal>(_: &T) {}

fn principal_ptr(p: &dyn SecurityPrincipal) -> *const () {
    p as *const dyn SecurityPrincipal as *const ()
}

fn security_ptr(security: &SecurityContext) -> *const () {
    security as *const SecurityContext as *const ()
}

/// @covers: SecurityPrincipal — SecurityContext satisfies the local trait bound
#[test]
fn test_security_context_implements_security_principal_happy() {
    let security = SecurityContext::unauthenticated();
    assert_security_principal(&security);
    let principal: &dyn SecurityPrincipal = &security;
    assert_eq!(principal_ptr(principal), security_ptr(&security));
}

/// @covers: HandlerContext::security — carries the bridged principal through unchanged
#[test]
fn test_handler_context_accepts_security_context_as_principal_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let observer_adapter = ObserverContextAdapter(observer.as_ref());
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: &observer_adapter,
    };
    assert_eq!(principal_ptr(ctx.security), security_ptr(&security));
}

/// @covers: SecurityPrincipal — distinct SecurityContext values bridge to distinct pointers
#[test]
fn test_distinct_security_contexts_yield_distinct_principal_pointers_edge() {
    let a = SecurityContext::unauthenticated();
    let b = SecurityContext::unauthenticated();
    let principal_a: &dyn SecurityPrincipal = &a;
    let principal_b: &dyn SecurityPrincipal = &b;
    assert_ne!(principal_ptr(principal_a), principal_ptr(principal_b));
}
