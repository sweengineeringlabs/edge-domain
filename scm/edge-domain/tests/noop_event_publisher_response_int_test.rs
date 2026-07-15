//! Integration tests for `NoopEventPublisherResponse`.
#![cfg(feature = "event")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    Domain, DomainEvent, DomainRuntime, EventPublisherPublishRequest, NoopEventPublisherRequest,
};

struct AnyEvent;
impl DomainEvent for AnyEvent {}

/// @covers: NoopEventPublisherResponse
#[test]
fn test_noop_event_publisher_response_publisher_field_publish_happy() {
    futures::executor::block_on(async {
        let resp = Domain
            .noop_event_publisher(NoopEventPublisherRequest)
            .unwrap();
        assert_eq!(
            resp.publisher
                .publish(EventPublisherPublishRequest { event: &AnyEvent })
                .await,
            Ok(())
        );
    });
}

/// @covers: NoopEventPublisherResponse
#[test]
fn test_noop_event_publisher_response_repeated_publish_never_errors_edge() {
    futures::executor::block_on(async {
        let resp = Domain
            .noop_event_publisher(NoopEventPublisherRequest)
            .unwrap();
        for _ in 0..5 {
            assert_eq!(
                resp.publisher
                    .publish(EventPublisherPublishRequest { event: &AnyEvent })
                    .await,
                Ok(())
            );
        }
    });
}

/// @covers: NoopEventPublisherResponse
#[test]
fn test_noop_event_publisher_response_publisher_is_uniquely_owned_error() {
    let resp = Domain
        .noop_event_publisher(NoopEventPublisherRequest)
        .unwrap();
    assert_eq!(std::sync::Arc::strong_count(&resp.publisher), 1);
}
