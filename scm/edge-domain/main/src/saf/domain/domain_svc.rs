//! Domain factory methods — all building-block constructors as methods on [`Domain`].
/// SAF module anchor — satisfies arch-audit rule 221.
pub const DOMAIN_SVC: () = ();

use std::hash::Hash;
use std::sync::Arc;

use crate::api::Domain;

#[cfg(feature = "command")]
use crate::api::CommandBus;
#[cfg(feature = "command")]
use edge_domain_command::DirectCommandBus;

#[cfg(feature = "event")]
use crate::api::Aggregate;
#[cfg(feature = "event")]
use crate::api::DomainEvent;
#[cfg(feature = "event")]
use crate::api::EventBus;
#[cfg(feature = "event")]
use crate::api::EventBusConfig;
#[cfg(feature = "event")]
use crate::api::EventPublisher;
#[cfg(feature = "event")]
use crate::api::EventStore;
#[cfg(feature = "event")]
use crate::api::EventStoreError;
#[cfg(feature = "event")]
use crate::api::InMemoryEventStore;
#[cfg(feature = "event")]
use crate::api::NoopEventBus;
#[cfg(feature = "event")]
use crate::api::NoopEventPublisher;
#[cfg(feature = "event")]
use edge_domain_event::InProcessEventBus;

#[cfg(feature = "handler")]
use crate::api::EchoHandler;
#[cfg(feature = "handler")]
use crate::api::Handler;
#[cfg(feature = "handler")]
use crate::api::HandlerRegistry as HandlerRegistryTrait;
#[cfg(feature = "handler")]
use crate::api::InProcessHandlerRegistry;

#[cfg(feature = "projection")]
use crate::api::Projection;
#[cfg(feature = "projection")]
use edge_domain_projection::InMemoryProjection;

#[cfg(feature = "query")]
use crate::api::DirectQueryBus;
#[cfg(feature = "query")]
use crate::api::QueryBus;

#[cfg(feature = "repository")]
use crate::api::InMemoryRepository;
#[cfg(feature = "repository")]
use crate::api::QueryableRepository;
#[cfg(feature = "repository")]
use crate::api::Repository;

#[cfg(feature = "saga")]
use crate::api::InMemorySagaStore;
#[cfg(feature = "saga")]
use crate::api::Saga;
#[cfg(feature = "saga")]
use crate::api::SagaStore;

#[cfg(feature = "service")]
use crate::api::ServiceRegistry as ServiceRegistryTrait;
#[cfg(feature = "service")]
use crate::api::ServiceRegistryImpl;

#[cfg(feature = "snapshot")]
use crate::api::Snapshot;
#[cfg(feature = "snapshot")]
use crate::api::SnapshotStore;
#[cfg(feature = "snapshot")]
use edge_domain_snapshot::InMemorySnapshotStore;

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

    /// Construct a paired `(T, U)` from a shared backend `Arc<B>`.
    ///
    /// Both closures receive `Arc::clone(&backend)`, ensuring writes through
    /// `T` are visible to reads through `U` when both share an in-memory
    /// store. Without this, two separate `from_config()` calls create
    /// independent backend instances and writes are invisible across them.
    pub fn paired<B: ?Sized, T, U>(
        backend: Arc<B>,
        make_first: impl FnOnce(Arc<B>) -> T,
        make_second: impl FnOnce(Arc<B>) -> U,
    ) -> (T, U) {
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
        let r = ServiceRegistryImpl::new();
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

    /// Construct a [`CommandBus`] that dispatches commands inline.
    #[cfg(feature = "command")]
    pub fn direct_command_bus() -> Arc<dyn CommandBus> {
        let b = DirectCommandBus;
        Arc::new(b)
    }

    /// Construct an [`EventPublisher`] that discards all events silently.
    #[cfg(feature = "event")]
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
    #[cfg(feature = "projection")]
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
    #[cfg(feature = "snapshot")]
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

    /// Construct a [`QueryBus`] that dispatches queries inline.
    #[cfg(feature = "query")]
    pub fn direct_query_bus<R: Send + 'static>() -> Arc<dyn QueryBus<Result = R>> {
        let b = DirectQueryBus::<R>::new();
        Arc::new(b)
    }

    /// Construct an in-process broadcast-backed [`EventBus`].
    ///
    /// Backed by the tokio broadcast implementation in `edge-domain-event`.
    #[cfg(feature = "event")]
    pub fn in_process_event_bus(config: EventBusConfig) -> Arc<dyn EventBus> {
        let b = InProcessEventBus::new(config.capacity);
        Arc::new(b)
    }

    /// Construct an [`EventBus`] that silently discards all events.
    #[cfg(feature = "event")]
    pub fn noop_event_bus() -> Arc<dyn EventBus> {
        let b = NoopEventBus;
        Arc::new(b)
    }

    /// Validate a configuration value using its [`Validator`](crate::api::Validator) impl.
    #[cfg(feature = "validator")]
    pub fn validate_config<V: crate::api::Validator>(
        config: &V,
    ) -> Result<(), String> {
        config.validate().map_err(|e| e.to_string())
    }
}
