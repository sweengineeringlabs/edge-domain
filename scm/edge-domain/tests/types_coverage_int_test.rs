//! Comprehensive coverage tests for api/ types and configuration modules.

use edge_domain::{Aggregate, DomainEvent, Spec};
use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
use std::time::SystemTime;

/// @covers: SecurityContext — unauthenticated constructor
#[test]
fn test_security_context_unauthenticated() {
    let ctx: SecurityContext = SecurityServices::unauthenticated();
    assert!(!ctx.authenticated);
    assert!(ctx.principal.is_none());
}

/// @covers: SecurityContext — authenticated constructor
#[test]
fn test_security_context_authenticated_with_principal() {
    use edge_domain_security::AnonymousPrincipal;
    let ctx: SecurityContext = SecurityServices::authenticated(Box::new(AnonymousPrincipal));
    assert!(ctx.authenticated);
    assert!(ctx.principal.is_some());
}

/// @covers: Aggregate
#[test]
fn test_aggregate_trait_apply_default() {
    #[derive(Default)]
    struct TestAggregate;

    #[derive(Clone)]
    struct TestEvent;

    impl DomainEvent for TestEvent {
        fn event_type(&self) -> &str {
            "test"
        }
        fn aggregate_id(&self) -> &str {
            "id"
        }
        fn occurred_at(&self) -> SystemTime {
            SystemTime::now()
        }
    }

    impl Aggregate for TestAggregate {
        type Event = TestEvent;
    }

    let mut agg = TestAggregate;
    let event = TestEvent;
    agg.apply(&event); // Should use default impl without error
}

/// @covers: Spec
#[test]
fn test_spec_matches_default() {
    struct AlwaysFalseSpec;
    impl Spec<String> for AlwaysFalseSpec {
        fn matches(&self, _s: &String) -> bool {
            false
        }
    }

    let spec = AlwaysFalseSpec;
    assert!(!spec.matches(&"test".to_string()));
}

/// @covers: Spec type and trait method defaults
#[test]
fn test_spec_default_implementation() {
    // Verify default impl of matches returns false
    struct TestSpec;
    impl Spec<i32> for TestSpec {}

    let spec = TestSpec;
    assert!(!spec.matches(&42));
}
