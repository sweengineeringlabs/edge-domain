//! Domain factory methods — all building-block constructors as methods on [`Domain`].

use std::hash::Hash;
use std::sync::Arc;

use crate::api::command::CommandBus;
use crate::api::event::Aggregate;
use crate::api::event::DomainEvent;
use crate::api::event::EventBus;
use crate::api::event::EventBusConfig;
use crate::api::event::EventPublisher;
use crate::api::event::EventStore;
use crate::api::event::EventStoreError;
use edge_dispatch::Dispatch;
use edge_dispatch::Handler;
use edge_dispatch::HandlerRegistry as HandlerRegistryTrait;
use crate::api::query::QueryBus;
use crate::api::repository::QueryableRepository;
use crate::api::repository::Repository;
use crate::api::service::types::ServiceRegistry;
use crate::api::service::ServiceRegistry as ServiceRegistryTrait;
use crate::api::types::Domain;
use crate::core::command::direct_command_bus::DirectCommandBus;
use crate::core::event::in_memory_event_store::InMemoryEventStore;
use crate::core::event::noop_event_bus::NoopEventBus;
use crate::core::event::noop_event_publisher::NoopEventPublisher;
use crate::core::query::direct_query_bus::DirectQueryBus;
use crate::core::repository::in_memory_repository::InMemoryRepository;
use crate::spi::event::tokio::tokio_event_bus::TokioEventBus;

impl Domain {
    /// Construct an [`EchoHandler`] that returns its input as its output.
    ///
    /// Useful for transport-layer integration tests — verifies routing and codec
    /// wiring without requiring any business logic.
    pub fn echo_handler<T>(
        id: impl Into<String>,
        pattern: impl Into<String>,
    ) -> Arc<dyn Handler<T, T>>
    where
        T: Send + 'static,
    {
        Dispatch::echo_handler(id, pattern)
    }

    /// Construct a fresh empty [`HandlerRegistry`].
    pub fn new_handler_registry<Request, Response>(
    ) -> Arc<dyn HandlerRegistryTrait<Request, Response>>
    where
        Request: Send + 'static,
        Response: Send + 'static,
    {
        Dispatch::new_handler_registry()
    }

    /// Construct a paired `(H1, H2)` from a shared backend `Arc<B>`.
    ///
    /// Both closures receive `Arc::clone(&backend)`, ensuring writes through
    /// `H1` are visible to reads through `H2` when both share an in-memory
    /// store. Without this, two separate `from_config()` calls create
    /// independent backend instances and writes are invisible across them.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::sync::Arc;
    /// use edge_domain::{Domain, Repository};
    ///
    /// struct WriteHandler { repo: Arc<dyn Repository<String, String>> }
    /// struct ReadHandler  { repo: Arc<dyn Repository<String, String>> }
    ///
    /// let (writer, reader) = Domain::paired(
    ///     Domain::new_in_memory_repository::<String, String>(),
    ///     |repo| WriteHandler { repo },
    ///     |repo| ReadHandler  { repo },
    /// );
    /// ```
    pub fn paired<B: ?Sized, H1, H2>(
        backend: Arc<B>,
        make_first: impl FnOnce(Arc<B>) -> H1,
        make_second: impl FnOnce(Arc<B>) -> H2,
    ) -> (H1, H2) {
        let first = make_first(Arc::clone(&backend));
        let second = make_second(backend);
        (first, second)
    }

    /// Construct a fresh empty [`ServiceRegistry`].
    pub fn new_service_registry<Request, Response>(
    ) -> Arc<dyn ServiceRegistryTrait<Request, Response>>
    where
        Request: Send + 'static,
        Response: Send + 'static,
    {
        let r = ServiceRegistry::new();
        Arc::new(r)
    }

    /// Construct a thread-safe in-memory [`Repository`].
    pub fn new_in_memory_repository<T, Id>() -> Arc<dyn Repository<T, Id>>
    where
        Id: Hash + Eq + Clone + Send + Sync + 'static,
        T: Clone + Send + Sync + 'static,
    {
        let r = InMemoryRepository::new();
        Arc::new(r)
    }

    /// Construct a thread-safe in-memory [`QueryableRepository`].
    pub fn new_in_memory_queryable_repository<T, Id>() -> Arc<dyn QueryableRepository<T, Id>>
    where
        Id: Hash + Eq + Clone + Send + Sync + 'static,
        T: Clone + Send + Sync + 'static,
    {
        let r = InMemoryRepository::new();
        Arc::new(r)
    }

    /// Construct a [`CommandBus`] that dispatches commands inline.
    pub fn direct_command_bus() -> Arc<dyn CommandBus> {
        let b = DirectCommandBus;
        Arc::new(b)
    }

    /// Construct an [`EventPublisher`] that discards all events silently.
    pub fn noop_event_publisher() -> Arc<dyn EventPublisher> {
        let p = NoopEventPublisher;
        Arc::new(p)
    }

    /// Construct a thread-safe in-memory [`EventStore`].
    pub fn new_in_memory_event_store<E>() -> Arc<dyn EventStore<E>>
    where
        E: DomainEvent + Send + Sync + Clone + 'static,
    {
        let s = InMemoryEventStore::new();
        Arc::new(s)
    }

    /// Reconstitute an aggregate by replaying all events from an [`EventStore`].
    ///
    /// Returns `None` when no events exist for `aggregate_id`.
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
        let b = DirectQueryBus;
        Arc::new(b)
    }

    /// Construct an in-process broadcast-backed [`EventBus`].
    ///
    /// Backed by the tokio broadcast implementation in `spi/event/tokio`.
    pub fn in_process_event_bus(config: EventBusConfig) -> Arc<dyn EventBus> {
        let b = TokioEventBus::new(config);
        Arc::new(b)
    }

    /// Construct an [`EventBus`] that silently discards all events.
    pub fn noop_event_bus() -> Arc<dyn EventBus> {
        let b = NoopEventBus;
        Arc::new(b)
    }

    /// Validate a configuration value using its [`Validator`](crate::api::traits::Validator) impl.
    pub fn validate_config<V: crate::api::traits::Validator>(config: &V) -> Result<(), String> {
        config.validate()
    }
}
