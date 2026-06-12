//! SAF layer — domain public facade.

mod aggregate_svc;
mod clock;
mod command;
mod domain;
mod entity_svc;
mod event;
mod handler;
mod policy;
mod policy_svc;
mod projection_svc;
mod query;
mod queryable_repository_svc;
mod repository;
mod repository_svc;
mod saga;
mod service;
mod snapshot;
mod spec_svc;
mod validator_svc;
mod value;
mod value_object_svc;

// ── domain (never extracted — always from internal api/) ────────────────────
pub use crate::api::domain::traits::DomainExtension;
pub use crate::api::domain::traits::DomainFactory;
pub use crate::api::domain::types::Domain;
pub use crate::api::domain::types::NoopDomainExtension;
pub use crate::api::domain::types::OutboundRegistry;

// ── entity ───────────────────────────────────────────────────────────────────
#[cfg(feature = "entity")]
pub use edge_domain_entity::Entity;
#[cfg(not(feature = "entity"))]
pub use crate::api::entity::Entity;

// ── valueobject ──────────────────────────────────────────────────────────────
#[cfg(feature = "valueobject")]
pub use edge_domain_valueobject::NonEmptyString;
#[cfg(feature = "valueobject")]
pub use edge_domain_valueobject::ValueObject;
#[cfg(not(feature = "valueobject"))]
pub use crate::api::valueobject::NonEmptyString;
#[cfg(not(feature = "valueobject"))]
pub use crate::api::valueobject::ValueObject;

// ── clock ────────────────────────────────────────────────────────────────────
#[cfg(feature = "clock")]
pub use edge_domain_clock::Clock;
#[cfg(feature = "clock")]
pub use edge_domain_clock::ClockError;
#[cfg(feature = "clock")]
pub use edge_domain_clock::ClockFactory;
#[cfg(feature = "clock")]
pub use edge_domain_clock::FixedClock;
#[cfg(feature = "clock")]
pub use edge_domain_clock::SystemClock;
#[cfg(not(feature = "clock"))]
pub use crate::api::clock::Clock;
#[cfg(not(feature = "clock"))]
pub use crate::api::clock::FixedClock;
#[cfg(not(feature = "clock"))]
pub use crate::api::clock::SystemClock;

// ── validator ─────────────────────────────────────────────────────────────────
#[cfg(feature = "validator")]
pub use edge_domain_validator::AlwaysValid;
#[cfg(feature = "validator")]
pub use edge_domain_validator::Validator;
#[cfg(feature = "validator")]
pub use edge_domain_validator::ValidatorError;
#[cfg(feature = "validator")]
pub use edge_domain_validator::ValidatorFactory;
#[cfg(not(feature = "validator"))]
pub use crate::api::validator::traits::Validator;

// ── policy ────────────────────────────────────────────────────────────────────
#[cfg(feature = "policy")]
pub use edge_domain_policy::CompositePolicy;
#[cfg(feature = "policy")]
pub use edge_domain_policy::Policy;
#[cfg(feature = "policy")]
pub use edge_domain_policy::PolicyFactory;
#[cfg(feature = "policy")]
pub use edge_domain_policy::PolicyViolation;
#[cfg(not(feature = "policy"))]
pub use crate::api::policy::CompositePolicy;
#[cfg(not(feature = "policy"))]
pub use crate::api::policy::Policy;
#[cfg(not(feature = "policy"))]
pub use crate::api::policy::PolicyViolation;

// ── command ───────────────────────────────────────────────────────────────────
#[cfg(feature = "command")]
pub use edge_domain_command::Command;
#[cfg(feature = "command")]
pub use edge_domain_command::CommandBus;
#[cfg(feature = "command")]
pub use edge_domain_command::CommandBusFactory;
#[cfg(feature = "command")]
pub use edge_domain_command::CommandError;
#[cfg(feature = "command")]
pub use edge_domain_command::DirectCommandBus;
#[cfg(not(feature = "command"))]
pub use crate::api::command::Command;
#[cfg(not(feature = "command"))]
pub use crate::api::command::CommandBus;
#[cfg(not(feature = "command"))]
pub use crate::api::command::CommandError;
#[cfg(not(feature = "command"))]
pub use crate::api::command::DirectCommandBus;

// ── query ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "query")]
pub use edge_domain_query::DirectQueryBus;
#[cfg(feature = "query")]
pub use edge_domain_query::Query;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryBus;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryBusFactory;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryError;
#[cfg(not(feature = "query"))]
pub use crate::api::query::DirectQueryBus;
#[cfg(not(feature = "query"))]
pub use crate::api::query::Query;
#[cfg(not(feature = "query"))]
pub use crate::api::query::QueryBus;
#[cfg(not(feature = "query"))]
pub use crate::api::query::QueryError;

// ── snapshot ──────────────────────────────────────────────────────────────────
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::InMemorySnapshotStore;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::Snapshot;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotError;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotStore;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotStoreFactory;
#[cfg(not(feature = "snapshot"))]
pub use crate::api::snapshot::Snapshot;
#[cfg(not(feature = "snapshot"))]
pub use crate::api::snapshot::SnapshotError;
#[cfg(not(feature = "snapshot"))]
pub use crate::api::snapshot::SnapshotStore;

// ── service ───────────────────────────────────────────────────────────────────
#[cfg(feature = "service")]
pub use edge_domain_service::Service;
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceError;
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceRegistry;
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceRegistryFactory;
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceRegistryImpl;
#[cfg(not(feature = "service"))]
pub use crate::api::service::Service;
#[cfg(not(feature = "service"))]
pub use crate::api::service::ServiceError;
#[cfg(not(feature = "service"))]
pub use crate::api::service::types::ServiceRegistry;

// ── repository ────────────────────────────────────────────────────────────────
#[cfg(feature = "repository")]
pub use edge_domain_repository::InMemoryRepository;
#[cfg(feature = "repository")]
pub use edge_domain_repository::Page;
#[cfg(feature = "repository")]
pub use edge_domain_repository::QueryableRepository;
#[cfg(feature = "repository")]
pub use edge_domain_repository::Repository;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryError;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryFactory;
#[cfg(feature = "repository")]
pub use edge_domain_repository::Spec;
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::{InMemoryRepository, Page};
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::QueryableRepository;
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::Repository;
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::RepositoryError;
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::Spec;

// ── handler ───────────────────────────────────────────────────────────────────
#[cfg(feature = "handler")]
pub use edge_domain_handler::EchoHandler;
#[cfg(feature = "handler")]
pub use edge_domain_handler::Handler;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerError;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerFactory;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerProvider;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerRegistry;
#[cfg(feature = "handler")]
pub use edge_domain_handler::InProcessHandlerRegistry;
#[cfg(feature = "handler")]
pub use edge_domain_handler::RequestContext;
#[cfg(feature = "handler")]
pub use edge_domain_handler::RequestContextBuilder;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::EchoHandler;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::Handler;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::HandlerError;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::HandlerFactory;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::HandlerRegistry;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::InProcessHandlerRegistry;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::RequestContext;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::RequestContextBuilder;

// ── event ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "event")]
pub use edge_domain_event::Aggregate;
#[cfg(feature = "event")]
pub use edge_domain_event::ClosedEventSource;
#[cfg(feature = "event")]
pub use edge_domain_event::DomainEvent;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBus;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBusConfig;
#[cfg(feature = "event")]
pub use edge_domain_event::EventEnvelope;
#[cfg(feature = "event")]
pub use edge_domain_event::EventError;
#[cfg(feature = "event")]
pub use edge_domain_event::EventFactory;
#[cfg(feature = "event")]
pub use edge_domain_event::EventPublisher;
#[cfg(feature = "event")]
pub use edge_domain_event::EventReceiver;
#[cfg(feature = "event")]
pub use edge_domain_event::EventSource;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStore;
#[cfg(feature = "event")]
pub use edge_domain_event::EventStoreError;
#[cfg(feature = "event")]
pub use edge_domain_event::ExpectedVersion;
#[cfg(feature = "event")]
pub use edge_domain_event::InMemoryEventStore;
#[cfg(feature = "event")]
pub use edge_domain_event::InProcessEventBus;
#[cfg(feature = "event")]
pub use edge_domain_event::NoopEventBus;
#[cfg(feature = "event")]
pub use edge_domain_event::NoopEventPublisher;
#[cfg(not(feature = "event"))]
pub use crate::api::event::Aggregate;
#[cfg(not(feature = "event"))]
pub use crate::api::event::ClosedEventSource;
#[cfg(not(feature = "event"))]
pub use crate::api::event::DomainEvent;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventBus;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventBusConfig;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventEnvelope;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventError;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventPublisher;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventReceiver;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventStore;
#[cfg(not(feature = "event"))]
pub use crate::api::event::EventStoreError;
#[cfg(not(feature = "event"))]
pub use crate::api::event::ExpectedVersion;
#[cfg(not(feature = "event"))]
pub use crate::api::event::{InMemoryEventStore, InProcessEventBus, NoopEventBus, NoopEventPublisher};

// ── projection (feature propagates "event", so DomainEvent comes from event gate) ──
#[cfg(feature = "projection")]
pub use edge_domain_projection::InMemoryProjection;
#[cfg(feature = "projection")]
pub use edge_domain_projection::Projection;
#[cfg(feature = "projection")]
pub use edge_domain_projection::ProjectionError;
#[cfg(feature = "projection")]
pub use edge_domain_projection::ProjectionFactory;
#[cfg(not(feature = "projection"))]
pub use crate::api::projection::Projection;

// ── saga (feature propagates "event" + "command"; only native items exported) ─
#[cfg(feature = "saga")]
pub use edge_domain_saga::InMemorySagaRegistry;
#[cfg(feature = "saga")]
pub use edge_domain_saga::Saga;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaError;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaFactory;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaRegistry;
#[cfg(not(feature = "saga"))]
pub use crate::api::saga::Saga;
#[cfg(not(feature = "saga"))]
pub use crate::api::saga::SagaError;
#[cfg(not(feature = "saga"))]
pub use crate::api::saga::SagaRegistry;
