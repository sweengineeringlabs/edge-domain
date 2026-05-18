//! Factory functions for domain building blocks.

use std::hash::Hash;
use std::sync::Arc;

use crate::api::command::CommandBus;
use crate::api::event::event_store_error::EventStoreError;
use crate::api::event::Aggregate;
use crate::api::event::DomainEvent;
use crate::api::event::EventPublisher;
use crate::api::event::EventStore;
use crate::api::handler::echo_handler::EchoHandler;
use crate::api::handler::handler_registry::HandlerRegistry;
use crate::api::handler::Handler;
use crate::api::query::QueryBus;
use crate::api::queryable_repository::QueryableRepository;
use crate::api::repository::Repository;
use crate::api::service::ServiceRegistry;
use crate::core::command::direct_command_bus::DirectCommandBus;
use crate::core::event::in_memory_event_store::InMemoryEventStore;
use crate::core::event::noop_event_publisher::NoopEventPublisher;
use crate::core::query::direct_query_bus::DirectQueryBus;
use crate::core::repository::in_memory_repository::InMemoryRepository;

/// Construct an [`EchoHandler`] that returns its input as its output.
///
/// Useful for transport-layer integration tests — verifies routing and codec
/// wiring without requiring any business logic.
pub fn echo_handler<T>(id: impl Into<String>, pattern: impl Into<String>) -> Arc<dyn Handler<T, T>>
where
    T: Send + 'static,
{
    Arc::new(EchoHandler::new(id, pattern))
}

/// Construct a fresh empty [`HandlerRegistry`].
///
/// Returned as `Arc<_>` because the registry is typically shared between
/// a `Job` impl and operator tooling that lists or mutates the handler set.
pub fn new_handler_registry<Request, Response>() -> Arc<HandlerRegistry<Request, Response>>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    Arc::new(HandlerRegistry::new())
}

/// Construct a fresh empty [`ServiceRegistry`].
pub fn new_service_registry<Request, Response>() -> Arc<ServiceRegistry<Request, Response>>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    Arc::new(ServiceRegistry::new())
}

/// Construct a thread-safe in-memory [`Repository`].
///
/// Suitable for development and testing. Not for production persistence.
pub fn new_in_memory_repository<T, Id>() -> Arc<dyn Repository<T, Id>>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    Arc::new(InMemoryRepository::new())
}

/// Construct a thread-safe in-memory [`QueryableRepository`].
///
/// Supports specification-based queries via [`QueryableRepository::find_by`],
/// [`find_one_by`](QueryableRepository::find_one_by), and
/// [`count_by`](QueryableRepository::count_by).
pub fn new_in_memory_queryable_repository<T, Id>() -> Arc<dyn QueryableRepository<T, Id>>
where
    Id: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    Arc::new(InMemoryRepository::new())
}

/// Construct a [`CommandBus`] that dispatches commands inline.
pub fn direct_command_bus() -> Arc<dyn CommandBus> {
    Arc::new(DirectCommandBus)
}

/// Construct an [`EventPublisher`] that discards all events silently.
///
/// Use during development or in services that do not yet require
/// event publishing infrastructure.
pub fn noop_event_publisher() -> Arc<dyn EventPublisher> {
    Arc::new(NoopEventPublisher)
}

/// Construct a thread-safe in-memory [`EventStore`].
///
/// Suitable for development and testing. State is lost when the process stops.
///
/// ```rust,ignore
/// let store = swe_edge_domain::new_in_memory_event_store::<OrderEvent>();
/// store.append("order-1", vec![event], ExpectedVersion::NoStream).await?;
/// ```
pub fn new_in_memory_event_store<E>() -> Arc<dyn EventStore<E>>
where
    E: DomainEvent + Send + Sync + Clone + 'static,
{
    Arc::new(InMemoryEventStore::new())
}

/// Reconstitute an aggregate by replaying all events from an [`EventStore`].
///
/// Returns `None` when no events exist for `aggregate_id` (aggregate was never
/// created).  Returns `Some(aggregate)` with state rebuilt by calling
/// [`Aggregate::apply`] on every event in sequence order.
///
/// ```rust,ignore
/// let order = swe_edge_domain::reconstitute::<Order>(&*store, "order-1").await?;
/// ```
pub async fn reconstitute<A>(
    store: &dyn EventStore<A::Event>,
    aggregate_id: &str,
) -> Result<Option<A>, EventStoreError>
where
    A: Aggregate,
{
    let envelopes = store.load(aggregate_id).await?;
    if envelopes.is_empty() {
        return Ok(None);
    }
    let mut aggregate = A::default();
    for envelope in &envelopes {
        aggregate.apply(&envelope.event);
    }
    Ok(Some(aggregate))
}

/// Construct a [`QueryBus`] that dispatches queries inline.
pub fn direct_query_bus<R: Send + 'static>() -> Arc<dyn QueryBus<R>> {
    Arc::new(DirectQueryBus)
}

/// Validate a configuration value using its [`Validator`](crate::api::traits::Validator) implementation.
///
/// Returns `Ok(())` when valid, or `Err` with a human-readable description
/// of the first validation failure.
pub fn validate_config<V: crate::api::traits::Validator>(config: &V) -> Result<(), String> {
    config.validate()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: echo_handler
    #[test]
    fn test_echo_handler() {
        let _: Arc<dyn Handler<String, String>> = echo_handler("id", "/path");
    }

    /// @covers: echo_handler
    #[tokio::test]
    async fn test_echo_handler_returns_input_as_output() {
        let h = echo_handler::<String>("echo", "/echo");
        assert_eq!(h.execute("ping".into()).await.unwrap(), "ping");
    }

    /// @covers: new_handler_registry
    #[test]
    fn test_new_handler_registry_returns_empty_registry() {
        let reg: Arc<HandlerRegistry<String, String>> = new_handler_registry();
        assert!(reg.is_empty());
    }

    /// @covers: new_service_registry
    #[test]
    fn test_new_service_registry_returns_empty_registry() {
        let reg: Arc<ServiceRegistry<String, String>> = new_service_registry();
        assert!(reg.is_empty());
    }

    /// @covers: new_in_memory_repository
    #[test]
    fn test_new_in_memory_repository() {
        let _: Arc<dyn crate::api::repository::Repository<String, u32>> =
            new_in_memory_repository();
    }

    /// @covers: new_in_memory_queryable_repository
    #[test]
    fn test_new_in_memory_queryable_repository() {
        let _: Arc<dyn crate::api::queryable_repository::QueryableRepository<String, u32>> =
            new_in_memory_queryable_repository();
    }

    /// @covers: new_in_memory_queryable_repository
    #[tokio::test]
    async fn test_new_in_memory_queryable_repository_returns_functional_store() {
        use crate::api::spec::Spec;
        struct Any;
        impl Spec<String> for Any {
            fn matches(&self, _: &String) -> bool {
                true
            }
        }
        let repo = new_in_memory_queryable_repository::<String, u32>();
        repo.save(1u32, "x".to_string()).await.unwrap();
        assert_eq!(repo.count_by(&Any).await.unwrap(), 1);
    }

    /// @covers: new_in_memory_repository
    #[tokio::test]
    async fn test_new_in_memory_repository_saves_and_finds_entity() {
        let repo = new_in_memory_repository::<String, u32>();
        repo.save(1u32, "x".to_string()).await.unwrap();
        assert!(repo.find(&1u32).await.unwrap().is_some());
    }

    /// @covers: new_in_memory_queryable_repository
    #[tokio::test]
    async fn test_new_in_memory_queryable_repository_supports_count_by() {
        use crate::api::spec::Spec;
        struct Any;
        impl Spec<String> for Any {
            fn matches(&self, _: &String) -> bool {
                true
            }
        }
        let repo = new_in_memory_queryable_repository::<String, u32>();
        repo.save(1u32, "x".to_string()).await.unwrap();
        assert_eq!(repo.count_by(&Any).await.unwrap(), 1);
    }

    /// @covers: validate_config
    #[test]
    fn test_validate_config_returns_ok_for_valid_input() {
        use crate::api::traits::Validator;
        struct AlwaysValid;
        impl Validator for AlwaysValid {
            fn validate(&self) -> Result<(), String> {
                Ok(())
            }
        }
        assert!(validate_config(&AlwaysValid).is_ok());
    }

    /// @covers: validate_config
    #[test]
    fn test_validate_config_returns_err_for_invalid_input() {
        use crate::api::traits::Validator;
        struct AlwaysInvalid;
        impl Validator for AlwaysInvalid {
            fn validate(&self) -> Result<(), String> {
                Err("bad".into())
            }
        }
        assert!(validate_config(&AlwaysInvalid).is_err());
    }

    /// @covers: direct_command_bus
    #[test]
    fn test_direct_command_bus_returns_arc_command_bus() {
        let bus = direct_command_bus();
        let _: Arc<dyn CommandBus> = bus;
    }

    /// @covers: noop_event_publisher
    #[test]
    fn test_noop_event_publisher_returns_arc_event_publisher() {
        let pub_ = noop_event_publisher();
        let _: Arc<dyn EventPublisher> = pub_;
    }

    /// @covers: direct_query_bus
    #[test]
    fn test_direct_query_bus_returns_arc_query_bus() {
        let bus = direct_query_bus::<String>();
        let _: Arc<dyn QueryBus<String>> = bus;
    }
}
