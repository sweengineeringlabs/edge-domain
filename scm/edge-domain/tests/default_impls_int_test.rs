//! Integration tests for default domain implementations:
//! direct_command_bus, noop_event_publisher.

use edge_domain::{
    Command, CommandBus, CommandError, Domain, DomainEvent, EventError, EventPublisher,
};
use futures::future::BoxFuture;
use std::sync::Arc;
use std::time::SystemTime;

// ── fixtures ─────────────────────────────────────────────────────────────────

struct OkCommand;
impl Command for OkCommand {
    fn name(&self) -> &str {
        "ok"
    }
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct ErrCommand;
impl Command for ErrCommand {
    fn name(&self) -> &str {
        "err"
    }
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("blocked".into())) })
    }
}

struct AnyEvent;
impl DomainEvent for AnyEvent {
    fn event_type(&self) -> &str {
        "test.event"
    }
    fn aggregate_id(&self) -> &str {
        "agg-1"
    }
    fn occurred_at(&self) -> SystemTime {
        SystemTime::now()
    }
}

struct FailingPublisher;
impl EventPublisher for FailingPublisher {
    fn publish(&self, _: &dyn DomainEvent) -> BoxFuture<'_, Result<(), EventError>> {
        Box::pin(async { Err(EventError::Unavailable("bus down".into())) })
    }
}

// ── direct_command_bus ────────────────────────────────────────────────────────

/// @covers: direct_command_bus
#[tokio::test]
async fn test_direct_command_bus_dispatches_ok_command_successfully() {
    let bus: Arc<dyn CommandBus> = Domain::direct_command_bus();
    assert!(bus.dispatch(Box::new(OkCommand)).await.is_ok());
}

/// @covers: direct_command_bus
#[tokio::test]
async fn test_direct_command_bus_propagates_command_error() {
    let bus: Arc<dyn CommandBus> = Domain::direct_command_bus();
    assert!(bus.dispatch(Box::new(ErrCommand)).await.is_err());
}

// ── noop_event_publisher ──────────────────────────────────────────────────────

/// @covers: noop_event_publisher
#[tokio::test]
async fn test_noop_event_publisher_always_returns_ok() {
    let pub_: Arc<dyn EventPublisher> = Domain::noop_event_publisher();
    assert!(pub_.publish(&AnyEvent).await.is_ok());
}

/// @covers: EventPublisher — custom impl can return error
#[tokio::test]
async fn test_event_publisher_trait_custom_impl_can_return_error() {
    let pub_: Arc<dyn EventPublisher> = Arc::new(FailingPublisher);
    assert!(pub_.publish(&AnyEvent).await.is_err());
}
