//! Coverage for api/event/types/noop/noop_event_publisher.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]
use edge_domain::DomainRuntime;
use edge_domain::NoopEventPublisherRequest;
use edge_domain::{Domain, EventPublisherPublishRequest, NoopEventPublisher};
use futures::executor::block_on;

#[test]
fn test_noop_event_publisher_marker_type_is_constructible() {
    let marker = NoopEventPublisher;
    assert_eq!(std::mem::size_of_val(&marker), 0);
}

#[test]
fn test_noop_event_publisher_publish_always_succeeds() {
    block_on(async {
        use edge_domain::DomainEvent;
        struct AnyEvent;
        impl DomainEvent for AnyEvent {}
        let pub_ = Domain
            .noop_event_publisher(NoopEventPublisherRequest)
            .unwrap()
            .publisher;
        assert_eq!(
            pub_.publish(EventPublisherPublishRequest { event: &AnyEvent })
                .await,
            Ok(())
        );
    });
}

#[test]
fn test_noop_event_publisher_repeated_publish_never_errors() {
    block_on(async {
        use edge_domain::DomainEvent;
        struct AnyEvent;
        impl DomainEvent for AnyEvent {}
        let pub_ = Domain
            .noop_event_publisher(NoopEventPublisherRequest)
            .unwrap()
            .publisher;
        for _ in 0..5 {
            assert_eq!(
                pub_.publish(EventPublisherPublishRequest { event: &AnyEvent })
                    .await,
                Ok(())
            );
        }
    });
}
