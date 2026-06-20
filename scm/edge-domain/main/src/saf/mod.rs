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

// ── sub-module re-exports (all api/ exports flow through _svc.rs inside each subdir) ─
pub use self::clock::*;
pub use self::command::*;
pub use self::domain::*;
pub use self::event::*;
pub use self::handler::*;
pub use self::policy::*;
pub use self::query::*;
pub use self::repository::*;
pub use self::saga::*;
pub use self::service::*;
pub use self::snapshot::*;
pub use self::value::*;

// ── top-level _svc.rs re-exports ─────────────────────────────────────────────
pub use self::aggregate_svc::*;
pub use self::entity_svc::*;
pub use self::policy_svc::*;
pub use self::projection_svc::*;
pub use self::queryable_repository_svc::*;
pub use self::repository_svc::*;
pub use self::spec_svc::*;
pub use self::validator_svc::*;
pub use self::value_object_svc::*;

// ── entity (sub-crate when feature enabled) ───────────────────────────────────
#[cfg(feature = "entity")]
pub use edge_domain_entity::Entity;

// ── valueobject (sub-crate when feature enabled) ─────────────────────────────
#[cfg(feature = "valueobject")]
pub use edge_domain_valueobject::NonEmptyString;
#[cfg(feature = "valueobject")]
pub use edge_domain_valueobject::ValueObject;
#[cfg(feature = "valueobject")]
pub use edge_domain_valueobject::ValueObjectError;

// ── clock (sub-crate when feature enabled) ────────────────────────────────────
#[cfg(feature = "clock")]
pub use edge_domain_clock::Clock;
#[cfg(feature = "clock")]
pub use edge_domain_clock::ClockBootstrap;
#[cfg(feature = "clock")]
pub use edge_domain_clock::ClockError;
#[cfg(feature = "clock")]
pub use edge_domain_clock::FixedClock;
#[cfg(feature = "clock")]
pub use edge_domain_clock::SystemClock;

// ── validator (sub-crate when feature enabled) ────────────────────────────────
#[cfg(feature = "validator")]
pub use edge_domain_validator::AlwaysValid;
#[cfg(feature = "validator")]
pub use edge_domain_validator::Validator;
#[cfg(feature = "validator")]
pub use edge_domain_validator::ValidatorBootstrap;
#[cfg(feature = "validator")]
pub use edge_domain_validator::ValidatorError;

// ── policy (sub-crate when feature enabled) ───────────────────────────────────
#[cfg(feature = "policy")]
pub use edge_domain_policy::CompositePolicy;
#[cfg(feature = "policy")]
pub use edge_domain_policy::Policy;
#[cfg(feature = "policy")]
pub use edge_domain_policy::PolicyBootstrap;
#[cfg(feature = "policy")]
pub use edge_domain_policy::PolicyViolation;

// ── command (sub-crate when feature enabled) ──────────────────────────────────
#[cfg(feature = "command")]
pub use edge_domain_command::Command;
#[cfg(feature = "command")]
pub use edge_domain_command::CommandBus;
#[cfg(feature = "command")]
pub use edge_domain_command::CommandBusBootstrap;
#[cfg(feature = "command")]
pub use edge_domain_command::CommandError;
#[cfg(feature = "command")]
pub use edge_domain_command::DirectCommandBus;
#[cfg(feature = "command")]
pub use edge_domain_command::LoggingCommandBus;
#[cfg(feature = "command")]
pub use edge_domain_command::NoopCommandBus;

// ── query (sub-crate when feature enabled) ────────────────────────────────────
#[cfg(feature = "query")]
pub use edge_domain_query::DirectQueryBus;
#[cfg(feature = "query")]
pub use edge_domain_query::LoggingQueryBus;
#[cfg(feature = "query")]
pub use edge_domain_query::NoopQuery;
#[cfg(feature = "query")]
pub use edge_domain_query::NoopQueryBus;
#[cfg(feature = "query")]
pub use edge_domain_query::Query;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryBus;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryBusBootstrap;
#[cfg(feature = "query")]
pub use edge_domain_query::QueryError;
#[cfg(feature = "query")]
pub use edge_domain_query::StdQueryBusFactory;

// ── snapshot (sub-crate when feature enabled) ─────────────────────────────────
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::InMemorySnapshotStore;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::Snapshot;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotError;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotStore;
#[cfg(feature = "snapshot")]
pub use edge_domain_snapshot::SnapshotStoreBootstrap;

// ── service (sub-crate when feature enabled) ──────────────────────────────────
#[cfg(feature = "service")]
pub use edge_domain_service::Service;
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceError;
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceRegistry;
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceRegistryBootstrap;
#[cfg(feature = "service")]
pub use edge_domain_service::ServiceRegistryImpl;

// ── repository (sub-crate when feature enabled) ───────────────────────────────
#[cfg(feature = "repository")]
pub use edge_domain_repository::InMemoryRepository;
#[cfg(feature = "repository")]
pub use edge_domain_repository::Page;
#[cfg(feature = "repository")]
pub use edge_domain_repository::QueryableRepository;
#[cfg(feature = "repository")]
pub use edge_domain_repository::Repository;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryBootstrap;
#[cfg(feature = "repository")]
pub use edge_domain_repository::RepositoryError;
#[cfg(feature = "repository")]
pub use edge_domain_repository::Spec;

// ── handler (sub-crate when feature enabled) ──────────────────────────────────
#[cfg(feature = "handler")]
pub use edge_domain_handler::EchoHandler;
#[cfg(feature = "handler")]
pub use edge_domain_handler::Handler;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerBootstrap;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerContext;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerError;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerProvider;
#[cfg(feature = "handler")]
pub use edge_domain_handler::HandlerRegistry;
#[cfg(feature = "handler")]
pub use edge_domain_handler::InProcessHandlerRegistry;

// ── event (sub-crate when feature enabled) ────────────────────────────────────
#[cfg(feature = "event")]
pub use edge_domain_event::Aggregate;
#[cfg(feature = "event")]
pub use edge_domain_event::ClosedEventSource;
#[cfg(feature = "event")]
pub use edge_domain_event::DomainEvent;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBootstrap;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBus;
#[cfg(feature = "event")]
pub use edge_domain_event::EventBusConfig;
#[cfg(feature = "event")]
pub use edge_domain_event::EventEnvelope;
#[cfg(feature = "event")]
pub use edge_domain_event::EventError;
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

// ── projection (sub-crate when feature enabled) ───────────────────────────────
#[cfg(feature = "projection")]
pub use edge_domain_projection::InMemoryProjection;
#[cfg(feature = "projection")]
pub use edge_domain_projection::Projection;
#[cfg(feature = "projection")]
pub use edge_domain_projection::ProjectionBootstrap;
#[cfg(feature = "projection")]
pub use edge_domain_projection::ProjectionError;

// ── saga (sub-crate when feature enabled) ─────────────────────────────────────
#[cfg(feature = "saga")]
pub use edge_domain_saga::InMemorySagaStore;
#[cfg(feature = "saga")]
pub use edge_domain_saga::Saga;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaBootstrap;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaError;
#[cfg(feature = "saga")]
pub use edge_domain_saga::SagaStore;

// ── registry (opt-in; NOT in default features) ────────────────────────────────
#[cfg(feature = "registry")]
pub use edge_domain_registry::InMemoryRegistry;
#[cfg(feature = "registry")]
pub use edge_domain_registry::Registry;
#[cfg(feature = "registry")]
pub use edge_domain_registry::RegistryBootstrap;
#[cfg(feature = "registry")]
pub use edge_domain_registry::RegistryError;
#[cfg(feature = "registry")]
pub use edge_domain_registry::StdRegistryFactory;
#[cfg(feature = "registry")]
pub use edge_domain_registry::REGISTRY_FACTORY_SVC;
#[cfg(feature = "registry")]
pub use edge_domain_registry::REGISTRY_SVC;

// ── lifecycle (opt-in; NOT in default features) ───────────────────────────────
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::Lifecycle;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::LifecycleBootstrap;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::LifecycleError;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::ManagedLifecycle;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::PermissivePolicy;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::StdLifecycleFactory;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::TransitionPolicy;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::LIFECYCLE_FACTORY_SVC;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::LIFECYCLE_SVC;
#[cfg(feature = "lifecycle")]
pub use edge_domain_lifecycle::TRANSITION_POLICY_SVC;

// ── security (opt-in; NOT in default features) ───────────────────────────────
#[cfg(feature = "security")]
pub use edge_domain_security::AnonymousPrincipal;
#[cfg(feature = "security")]
pub use edge_domain_security::NoopSecurity;
#[cfg(feature = "security")]
pub use edge_domain_security::Principal;
#[cfg(feature = "security")]
pub use edge_domain_security::Security;
#[cfg(feature = "security")]
pub use edge_domain_security::SecurityBootstrap;
#[cfg(feature = "security")]
pub use edge_domain_security::SecurityContext;
#[cfg(feature = "security")]
pub use edge_domain_security::SecurityContextBuilder;
#[cfg(feature = "security")]
pub use edge_domain_security::SecurityError;
#[cfg(feature = "security")]
pub use edge_domain_security::SecurityServices;
#[cfg(feature = "security")]
pub use edge_domain_security::ANONYMOUS;
#[cfg(feature = "security")]
pub use edge_domain_security::DEFAULT_SERVICES;
#[cfg(feature = "security")]
pub use edge_domain_security::NOOP_SECURITY;
