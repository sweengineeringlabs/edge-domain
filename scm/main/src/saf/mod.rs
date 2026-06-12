//! SAF layer — domain public facade.

mod aggregate_svc;
mod clock_svc;
mod command;
mod domain;
mod entity_svc;
mod event;
mod handler;
mod policy_svc;
mod projection_svc;
mod query;
mod queryable_repository_svc;
mod repository_svc;
mod saga;
mod service;
mod snapshot;
mod spec_svc;
mod validator_svc;
mod value_object_svc;

pub use crate::api::domain::types::Domain;

pub use crate::api::clock::Clock;
pub use crate::api::clock::FixedClock;
pub use crate::api::clock::SystemClock;
pub use crate::api::command::Command;
pub use crate::api::command::CommandBus;
pub use crate::api::command::CommandError;
pub use crate::api::entity::Entity;
pub use crate::api::event::Aggregate;
pub use crate::api::event::DomainEvent;
pub use crate::api::event::EventBus;
pub use crate::api::event::EventError;
pub use crate::api::event::EventPublisher;
pub use crate::api::event::EventStore;
pub use crate::api::event::EventStoreError;
pub use crate::api::event::ExpectedVersion;
pub use crate::api::handler::Handler;
pub use crate::api::handler::HandlerError;
pub use crate::api::handler::HandlerRegistry;
pub use crate::api::handler::RequestContext;
pub use crate::api::handler::RequestContextBuilder;
pub use crate::api::policy::CompositePolicy;
pub use crate::api::policy::Policy;
pub use crate::api::policy::PolicyViolation;
pub use crate::api::projection::Projection;
pub use crate::api::query::Query;
pub use crate::api::query::QueryBus;
pub use crate::api::query::QueryError;
pub use crate::api::repository::QueryableRepository;
pub use crate::api::repository::Repository;
pub use crate::api::repository::RepositoryError;
pub use crate::api::repository::Spec;
pub use crate::api::saga::Saga;
pub use crate::api::saga::SagaError;
pub use crate::api::saga::SagaRegistry;
pub use crate::api::service::Service;
pub use crate::api::service::ServiceError;
pub use crate::api::snapshot::Snapshot;
pub use crate::api::snapshot::SnapshotError;
pub use crate::api::snapshot::SnapshotStore;

pub use crate::api::command::DirectCommandBus;
pub use crate::api::domain::traits::DomainExtension;
pub use crate::api::domain::types::NoopDomainExtension;
pub use crate::api::domain::types::{ApplicationConfig, OutboundRegistry};
pub use crate::api::event::{
    EventBusConfig, EventEnvelope, EventReceiver, InMemoryEventStore, InProcessEventBus,
    NoopEventBus, NoopEventPublisher,
};
pub use crate::api::query::DirectQueryBus;
pub use crate::api::repository::{InMemoryRepository, Page};
pub use crate::api::service::types::ServiceRegistry;
pub use crate::api::validator::traits::Validator;
pub use crate::api::validator::ValidatorDefault;
pub use crate::api::valueobject::NonEmptyString;
pub use crate::api::valueobject::ValueObject;
