//! Integration tests for saf factory functions.

use edge_domain::{
    direct_command_bus, direct_query_bus, new_handler_registry, new_in_memory_queryable_repository,
    new_in_memory_repository, new_service_registry, noop_event_publisher, Command, CommandBus,
    CommandError, HandlerRegistry, QueryBus, QueryError, QueryableRepository, Repository,
    ServiceRegistry,
};
use std::sync::Arc;

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

/// @covers: new_in_memory_repository
#[test]
fn test_new_in_memory_repository_returns_arc_repository() {
    let _: Arc<dyn Repository<String, u32>> = new_in_memory_repository();
}

/// @covers: new_in_memory_queryable_repository
#[test]
fn test_new_in_memory_queryable_repository_returns_arc_queryable_repository() {
    let _: Arc<dyn QueryableRepository<String, u32>> = new_in_memory_queryable_repository();
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_saves_and_finds_entity() {
    let repo: Arc<dyn Repository<String, u32>> = new_in_memory_repository();
    repo.save(1u32, "hello".to_string()).await.unwrap();
    let found = repo.find(&1u32).await.unwrap();
    assert_eq!(found.as_deref(), Some("hello"));
}

/// @covers: new_in_memory_queryable_repository
#[tokio::test]
async fn test_new_in_memory_queryable_repository_finds_by_spec() {
    use edge_domain::Spec;
    struct LongStr;
    impl Spec<String> for LongStr {
        fn matches(&self, s: &String) -> bool {
            s.len() > 3
        }
    }
    let repo: Arc<dyn QueryableRepository<String, u32>> = new_in_memory_queryable_repository();
    repo.save(1u32, "hi".to_string()).await.unwrap();
    repo.save(2u32, "hello".to_string()).await.unwrap();
    let results = repo.find_by(&LongStr).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "hello");
}

/// @covers: direct_command_bus
#[tokio::test]
async fn test_factory_fn_direct_command_bus_dispatches_command_inline() {
    use futures::future::BoxFuture;
    struct PingCommand;
    impl Command for PingCommand {
        fn name(&self) -> &str {
            "ping"
        }
        fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }
    let bus: Arc<dyn CommandBus> = direct_command_bus();
    assert!(bus.dispatch(Box::new(PingCommand)).await.is_ok());
}

/// @covers: noop_event_publisher
#[tokio::test]
async fn test_factory_fn_noop_event_publisher_silently_discards_events() {
    use edge_domain::DomainEvent;
    use std::time::SystemTime;
    struct AnyEvent;
    impl DomainEvent for AnyEvent {
        fn event_type(&self) -> &str {
            "any"
        }
        fn aggregate_id(&self) -> &str {
            "id-1"
        }
        fn occurred_at(&self) -> SystemTime {
            SystemTime::now()
        }
    }
    let publisher = noop_event_publisher();
    assert!(publisher.publish(&AnyEvent).await.is_ok());
}

/// @covers: direct_query_bus
#[tokio::test]
async fn test_factory_fn_direct_query_bus_dispatches_query_inline() {
    use edge_domain::Query;
    use futures::future::BoxFuture;
    struct EchoQuery(String);
    impl Query<String> for EchoQuery {
        fn name(&self) -> &str {
            "echo"
        }
        fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
            let v = self.0.clone();
            Box::pin(async move { Ok(v) })
        }
    }
    let bus: Arc<dyn QueryBus<String>> = direct_query_bus();
    let result = bus
        .dispatch(Box::new(EchoQuery("pong".into())))
        .await
        .unwrap();
    assert_eq!(result, "pong");
}
