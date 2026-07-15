//! Comprehensive coverage tests for api/ types and configuration modules.
#![cfg(all(feature = "event", feature = "repository"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    Aggregate, AggregateApplyRequest, DomainEvent, EventAggregateIdRequest,
    EventAggregateIdResponse, EventOccurredAtRequest, EventOccurredAtResponse, EventTypeRequest,
    EventTypeResponse, RepositoryError, Spec, SpecMatchesRequest, SpecMatchesResponse,
};
use edge_security_runtime::SecurityContext;
use std::time::SystemTime;

/// @covers: SecurityContext — unauthenticated constructor
#[test]
fn test_security_context_unauthenticated() {
    let ctx: SecurityContext = SecurityContext::unauthenticated();
    assert!(!ctx.authenticated);
    assert!(ctx.principal.is_none());
}

/// @covers: SecurityContext — authenticated constructor
#[test]
fn test_security_context_authenticated_with_principal() {
    use edge_security_runtime::AnonymousPrincipal;
    let ctx: SecurityContext = SecurityContext::authenticated_with(Box::new(AnonymousPrincipal));
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
        fn event_type(
            &self,
            _req: EventTypeRequest,
        ) -> Result<EventTypeResponse<'_>, edge_application::EventError> {
            Ok(EventTypeResponse { event_type: "test" })
        }
        fn aggregate_id(
            &self,
            _req: EventAggregateIdRequest,
        ) -> Result<EventAggregateIdResponse<'_>, edge_application::EventError> {
            Ok(EventAggregateIdResponse { aggregate_id: "id" })
        }
        fn occurred_at(
            &self,
            _req: EventOccurredAtRequest,
        ) -> Result<EventOccurredAtResponse, edge_application::EventError> {
            Ok(EventOccurredAtResponse {
                occurred_at: SystemTime::now(),
            })
        }
    }

    impl Aggregate for TestAggregate {
        type Event = TestEvent;
    }

    let mut agg = TestAggregate;
    let event = TestEvent;
    agg.apply(AggregateApplyRequest { event: &event }).unwrap(); // Should use default impl without error
}

/// @covers: Spec
#[test]
fn test_spec_matches_default() {
    struct AlwaysFalseSpec;
    impl Spec for AlwaysFalseSpec {
        type Entity = String;

        fn matches(
            &self,
            _req: SpecMatchesRequest<'_, String>,
        ) -> Result<SpecMatchesResponse, RepositoryError> {
            Ok(SpecMatchesResponse { matches: false })
        }
    }

    let spec = AlwaysFalseSpec;
    let entity = "test".to_string();
    assert!(
        !spec
            .matches(SpecMatchesRequest { entity: &entity })
            .unwrap()
            .matches
    );
}

/// @covers: Spec type and trait method defaults
#[test]
fn test_spec_default_implementation() {
    // Verify default impl of matches returns false
    struct TestSpec;
    impl Spec for TestSpec {
        type Entity = i32;
    }

    let spec = TestSpec;
    assert!(
        !spec
            .matches(SpecMatchesRequest { entity: &42 })
            .unwrap()
            .matches
    );
}
