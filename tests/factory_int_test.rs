//! Integration tests for saf factory functions.

use std::sync::Arc;
use edge_domain::{
    direct_command_bus, direct_query_bus, in_memory_repository, new_handler_registry,
    new_service_registry, noop_event_publisher,
    Command, CommandBus, CommandError, HandlerRegistry, QueryBus, QueryError,
    Repository, ServiceRegistry,
};

/// @covers: new_handler_registry
#[test]
fn test_factory_fn_new_handler_registry_returns_empty_arc_registry() {
    let reg: Arc<HandlerRegistry<String, String>> = new_handler_registry();
    assert!(reg.is_empty());
    assert_eq!(reg.len(), 0);
}

/// @covers: new_service_registry
#[test]
fn test_factory_fn_new_service_registry_returns_empty_arc_registry() {
    let reg: Arc<ServiceRegistry<String, String>> = new_service_registry();
    assert!(reg.is_empty());
    assert_eq!(reg.len(), 0);
}

/// @covers: in_memory_repository
#[tokio::test]
async fn test_factory_fn_in_memory_repository_returns_working_repository() {
    let repo: Arc<dyn Repository<String, u32>> = in_memory_repository();
    repo.save(1u32, "hello".to_string()).await.unwrap();
    let found = repo.find(&1u32).await.unwrap();
    assert_eq!(found.as_deref(), Some("hello"));
}

/// @covers: direct_command_bus
#[tokio::test]
async fn test_factory_fn_direct_command_bus_dispatches_command_inline() {
    use async_trait::async_trait;
    struct PingCommand;
    #[async_trait]
    impl Command for PingCommand {
        fn name(&self) -> &str { "ping" }
        async fn execute(&self) -> Result<(), CommandError> { Ok(()) }
    }
    let bus: Arc<dyn CommandBus> = direct_command_bus();
    assert!(bus.dispatch(Box::new(PingCommand)).await.is_ok());
}

/// @covers: noop_event_publisher
#[tokio::test]
async fn test_factory_fn_noop_event_publisher_silently_discards_events() {
    use std::time::SystemTime;
    use edge_domain::DomainEvent;
    struct AnyEvent;
    impl DomainEvent for AnyEvent {
        fn event_type(&self)   -> &str       { "any" }
        fn aggregate_id(&self) -> &str       { "id-1" }
        fn occurred_at(&self)  -> SystemTime { SystemTime::now() }
    }
    let publisher = noop_event_publisher();
    assert!(publisher.publish(&AnyEvent).await.is_ok());
}

/// @covers: direct_query_bus
#[tokio::test]
async fn test_factory_fn_direct_query_bus_dispatches_query_inline() {
    use async_trait::async_trait;
    use edge_domain::Query;
    struct EchoQuery(String);
    #[async_trait]
    impl Query<String> for EchoQuery {
        fn name(&self) -> &str { "echo" }
        async fn execute(&self) -> Result<String, QueryError> { Ok(self.0.clone()) }
    }
    let bus: Arc<dyn QueryBus<String>> = direct_query_bus();
    let result = bus.dispatch(Box::new(EchoQuery("pong".into()))).await.unwrap();
    assert_eq!(result, "pong");
}
