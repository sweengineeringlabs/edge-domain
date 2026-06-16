//! Domain factory methods — all building-block constructors as methods on [`Domain`].
/// SAF module anchor — satisfies arch-audit rule 221.
pub const DOMAIN_SVC: () = ();

use std::hash::Hash;
use std::sync::Arc;

use crate::api::command::CommandBus;
use crate::api::domain::types::Domain;
use crate::api::event::Aggregate;
use crate::api::event::DomainEvent;
use crate::api::event::EventBus;
use crate::api::event::EventBusConfig;
use crate::api::event::EventPublisher;
use crate::api::event::EventStore;
use crate::api::event::EventStoreError;
use crate::api::handler::EchoHandler;
use crate::api::handler::Handler;
use crate::api::handler::HandlerRegistry as HandlerRegistryTrait;
use crate::api::projection::Projection;
use crate::api::query::QueryBus;
use crate::api::repository::QueryableRepository;
use crate::api::repository::Repository;
use crate::api::saga::Saga;
use crate::api::saga::SagaStore;
use crate::api::service::types::ServiceRegistry;
use crate::api::service::ServiceRegistry as ServiceRegistryTrait;
use crate::api::snapshot::Snapshot;
use crate::api::snapshot::SnapshotStore;
use crate::core::command::direct_command_bus::DirectCommandBus;
use crate::core::projection::in_memory_projection::InMemoryProjection;
use crate::core::snapshot::in_memory_snapshot_store::InMemorySnapshotStore;
use crate::spi::event::tokio::tokio_event_bus::TokioEventBus;

// When the `event` feature is enabled the sub-crate provides real implementations;
// when disabled the local core/ fallbacks are used.
#[cfg(feature = "event")]
use crate::api::event::InMemoryEventStore;
#[cfg(feature = "event")]
use crate::api::event::NoopEventBus;
#[cfg(feature = "event")]
use crate::api::event::NoopEventPublisher;
#[cfg(not(feature = "event"))]
use crate::core::event::in_memory_event_store::InMemoryEventStore;
#[cfg(not(feature = "event"))]
use crate::core::event::noop::noop_event_bus::NoopEventBus;
#[cfg(not(feature = "event"))]
use crate::core::event::noop::noop_event_publisher::NoopEventPublisher;

#[cfg(feature = "handler")]
use crate::api::handler::InProcessHandlerRegistry;
#[cfg(not(feature = "handler"))]
use crate::core::handler::in_process_handler_registry::InProcessHandlerRegistry;

#[cfg(feature = "query")]
use crate::api::query::DirectQueryBus;
#[cfg(not(feature = "query"))]
use crate::core::query::direct_query_bus::DirectQueryBus;

#[cfg(feature = "repository")]
use crate::api::repository::InMemoryRepository;
#[cfg(not(feature = "repository"))]
use crate::core::repository::in_memory_repository::InMemoryRepository;

#[cfg(feature = "saga")]
use crate::api::saga::InMemorySagaStore;
#[cfg(not(feature = "saga"))]
use crate::core::saga::in_memory_saga_store::InMemorySagaStore;

impl Domain {
    /// Construct a handler that returns its input unchanged.
    ///
    /// Useful for transport-layer integration tests — verifies routing and
    /// codec wiring without requiring any business logic implementation.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use edge_domain::Domain;
    ///
    /// let h = Domain::echo_handler::<String>("echo", "/ping");
    /// ```
    #[cfg(feature = "handler")]
    pub fn echo_handler<T>(
        id: impl Into<String>,
        pattern: impl Into<String>,
    ) -> Arc<dyn Handler<Request = T, Response = T>>
    where
        T: Clone + Send + 'static,
    {
        Arc::new(EchoHandler::new(id, pattern))
    }

    #[cfg(not(feature = "handler"))]
    pub fn echo_handler<T>(
        id: impl Into<String>,
        pattern: impl Into<String>,
    ) -> Arc<dyn Handler<Request = T, Response = T>>
    where
        T: Clone + Send + 'static,
    {
        Arc::new(EchoHandler::new(id, pattern))
    }

    /// Construct a fresh empty handler registry.
    ///
    /// Backed by a `parking_lot::RwLock<HashMap>` — safe for concurrent
    /// registration and lookup across threads.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use edge_domain::Domain;
    ///
    /// let registry = Domain::new_handler_registry::<String, String>();
    /// assert!(registry.is_empty());
    /// ```
    #[cfg(feature = "handler")]
    pub fn new_handler_registry<Req, Resp>() -> Arc<dyn HandlerRegistryTrait<Request = Req, Response = Resp>>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        Arc::new(InProcessHandlerRegistry::new())
    }

    #[cfg(not(feature = "handler"))]
    pub fn new_handler_registry<Req, Resp>() -> Arc<dyn HandlerRegistryTrait<Request = Req, Response = Resp>>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        Arc::new(InProcessHandlerRegistry::new())
    }

    /// Construct a paired `(H1, H2)` from a shared backend `Arc<B>`.
    ///
    /// Both closures receive `Arc::clone(&backend)`, ensuring writes through
    /// `H1` are visible to reads through `H2` when both share an in-memory
    /// store. Without this, two separate `from_config()` calls create
    /// independent backend instances and writes are invisible across them.
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
    #[cfg(feature = "service")]
    pub fn new_service_registry<Request, Response>(
    ) -> Arc<dyn ServiceRegistryTrait<Request = Request, Response = Response>>
    where
        Request: Send + 'static,
        Response: Send + 'static,
    {
        let r = ServiceRegistry::new();
        Arc::new(r)
    }

    #[cfg(not(feature = "service"))]
    pub fn new_service_registry<Request, Response>(
    ) -> Arc<dyn ServiceRegistryTrait<Request = Request, Response = Response>>
    where
        Request: Send + 'static,
        Response: Send + 'static,
    {
        let r = ServiceRegistry::new();
        Arc::new(r)
    }

    /// Construct a thread-safe in-memory [`Repository`].
    #[cfg(feature = "repository")]
    pub fn new_in_memory_repository<T, Id>() -> Arc<dyn Repository<Entity = T, Id = Id>>
    where
        Id: Hash + Eq + Clone + Send + Sync + 'static,
        T: Clone + Send + Sync + 'static,
    {
        let r = InMemoryRepository::new();
        Arc::new(r)
    }

    #[cfg(not(feature = "repository"))]
    pub fn new_in_memory_repository<T, Id>() -> Arc<dyn Repository<Entity = T, Id = Id>>
    where
        Id: Hash + Eq + Clone + Send + Sync + 'static,
        T: Clone + Send + Sync + 'static,
    {
        let r = InMemoryRepository::new();
        Arc::new(r)
    }

    /// Construct a thread-safe in-memory [`QueryableRepository`].
    #[cfg(feature = "repository")]
    pub fn new_in_memory_queryable_repository<T, Id>() -> Arc<dyn QueryableRepository<Entity = T, Id = Id>>
    where
        Id: Hash + Eq + Clone + Send + Sync + 'static,
        T: Clone + Send + Sync + 'static,
    {
        let r = InMemoryRepository::new();
        Arc::new(r)
    }

    #[cfg(not(feature = "repository"))]
    pub fn new_in_memory_queryable_repository<T, Id>() -> Arc<dyn QueryableRepository<Entity = T, Id = Id>>
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
    #[cfg(feature = "event")]
    pub fn new_in_memory_event_store<E>() -> Arc<dyn EventStore<Event = E>>
    where
        E: DomainEvent + Send + Sync + Clone + 'static,
    {
        let s = InMemoryEventStore::new();
        Arc::new(s)
    }

    #[cfg(not(feature = "event"))]
    pub fn new_in_memory_event_store<E>() -> Arc<dyn EventStore<Event = E>>
    where
        E: DomainEvent + Send + Sync + Clone + 'static,
    {
        let s = InMemoryEventStore::<E>::new();
        Arc::new(s)
    }

    /// Construct an in-memory [`Projection`] that folds events into a read model.
    ///
    /// `initial` seeds the read model; `reducer` is invoked once per applied
    /// event to update it.  The returned projection takes `&mut self` on
    /// [`apply`](Projection::apply) — wrap it in a `Mutex` for shared ownership.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut p = Domain::new_in_memory_projection::<OrderEvent, u64, _>(
    ///     0,
    ///     |total, env| *total += 1,
    /// );
    /// ```
    pub fn new_in_memory_projection<E, R, F>(
        initial: R,
        reducer: F,
    ) -> Box<dyn Projection<Event = E, ReadModel = R>>
    where
        E: DomainEvent + Send + Sync + 'static,
        R: Send + Sync + 'static,
        F: Fn(&mut R, &E) + Send + Sync + 'static,
    {
        Box::new(InMemoryProjection::new(initial, reducer))
    }

    /// Construct a fresh in-memory [`SagaStore`] for saga type `S`.
    ///
    /// Sagas are stored keyed by their [`SagaId`](Saga::SagaId).  The store
    /// returns [`SagaError`](crate::SagaError) on duplicate registration or
    /// missing lookup; dispatching the commands a saga emits remains the
    /// caller's responsibility.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut store = Domain::new_in_memory_saga_store::<OrderSaga>();
    /// store.register(order_id, OrderSaga::default())?;
    /// ```
    #[cfg(feature = "saga")]
    pub fn new_in_memory_saga_store<S>() -> Box<dyn SagaStore<SagaInstance = S>>
    where
        S: Saga + 'static,
        S::SagaId: std::fmt::Display + 'static,
    {
        Box::new(InMemorySagaStore::new())
    }

    #[cfg(not(feature = "saga"))]
    pub fn new_in_memory_saga_store<S>() -> Box<dyn SagaStore<SagaInstance = S>>
    where
        S: Saga + 'static,
        S::SagaId: std::fmt::Display + 'static,
    {
        Box::new(InMemorySagaStore::new())
    }

    /// Construct a thread-safe in-memory [`SnapshotStore`] for snapshot type `S`.
    ///
    /// Keeps the latest snapshot per aggregate.  `save` rejects snapshots at
    /// version `0` with [`SnapshotError::InvalidVersion`](crate::SnapshotError);
    /// `load` returns `Ok(None)` when no snapshot exists for an aggregate.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let store = Domain::new_in_memory_snapshot_store::<OrderSnapshot>();
    /// store.save(snapshot).await?;
    /// let restored = store.load(&order_id).await?;
    /// ```
    pub fn new_in_memory_snapshot_store<S>(
    ) -> Arc<dyn SnapshotStore<AggregateId = S::AggregateId, Snap = S>>
    where
        S: Snapshot + Clone + 'static,
        S::AggregateId: std::fmt::Display + 'static,
    {
        Arc::new(InMemorySnapshotStore::new())
    }

    /// Reconstitute an aggregate by replaying all events from an [`EventStore`].
    ///
    /// Returns `None` when no events exist for `aggregate_id`.
    #[cfg(feature = "event")]
    pub async fn reconstitute<A>(
        store: &dyn EventStore<Event = A::Event>,
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

    #[cfg(not(feature = "event"))]
    pub async fn reconstitute<A>(
        store: &dyn EventStore<Event = A::Event>,
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
    #[cfg(feature = "query")]
    pub fn direct_query_bus<R: Send + 'static>() -> Arc<dyn QueryBus<Result = R>> {
        let b = DirectQueryBus::<R>::new();
        Arc::new(b)
    }

    #[cfg(not(feature = "query"))]
    pub fn direct_query_bus<R: Send + 'static>() -> Arc<dyn QueryBus<Result = R>> {
        let b = DirectQueryBus::<R>::new();
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

    /// Validate a configuration value using its [`Validator`](crate::api::validator::traits::Validator) impl.
    pub fn validate_config<V: crate::api::validator::traits::Validator>(
        config: &V,
    ) -> Result<(), String> {
        config.validate().map_err(|e| e.to_string())
    }
}
