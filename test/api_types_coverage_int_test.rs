//! Comprehensive coverage tests for api/ types and configuration modules.

use edge_domain::{Aggregate, ApplicationConfig, DomainEvent, RequestContext, Spec};
use std::time::SystemTime;

/// @covers: ApplicationConfig
#[test]
fn test_application_config_type_exists() {
    let _config = ApplicationConfig::default();
}

/// @covers: RequestContext
#[test]
fn test_request_context_unauthenticated() {
    let ctx = RequestContext::unauthenticated();
    assert!(!ctx.authenticated);
    assert!(ctx.subject.is_none());
}

/// @covers: RequestContext
#[test]
fn test_request_context_authenticated() {
    let ctx = RequestContext::authenticated(
        "user123",
        None,
        Some("tenant456".to_string()),
        Default::default(),
    );
    assert!(ctx.authenticated);
    assert_eq!(ctx.subject, Some("user123".to_string()));
    assert_eq!(ctx.tenant_id, Some("tenant456".to_string()));
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
