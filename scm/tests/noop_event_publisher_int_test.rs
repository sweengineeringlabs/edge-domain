//! Coverage for api/event/types/noop/noop_event_publisher.rs
use edge_domain::{Domain, NoopEventPublisher};
use futures::executor::block_on;

#[test]
fn test_noop_event_publisher_marker_type_is_constructible() {
    let _marker = NoopEventPublisher;
}

#[test]
fn test_noop_event_publisher_publish_always_succeeds() {
    block_on(async {
        use edge_domain::DomainEvent;
        struct AnyEvent;
        impl DomainEvent for AnyEvent {}
        let pub_ = Domain::noop_event_publisher();
        assert!(pub_.publish(&AnyEvent).await.is_ok());
    });
}

#[test]
fn test_noop_event_publisher_repeated_publish_never_errors() {
    block_on(async {
        use edge_domain::DomainEvent;
        struct AnyEvent;
        impl DomainEvent for AnyEvent {}
        let pub_ = Domain::noop_event_publisher();
        for _ in 0..5 {
            assert!(pub_.publish(&AnyEvent).await.is_ok());
        }
    });
}
