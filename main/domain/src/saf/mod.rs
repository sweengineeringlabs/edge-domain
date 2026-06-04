//! SAF layer — domain public facade.

mod domain_svc;

pub use crate::api::types::Domain;

pub use crate::api::command::Command;
pub use crate::api::command::CommandBus;
pub use crate::api::command::CommandError;
pub use crate::api::error::HandlerError;
pub use crate::api::error::RepositoryError;
pub use crate::api::event::Aggregate;
pub use crate::api::event::DomainEvent;
pub use crate::api::event::EventBus;
pub use crate::api::event::EventError;
pub use crate::api::event::EventPublisher;
pub use crate::api::event::EventStore;
pub use crate::api::event::EventStoreError;
pub use crate::api::event::ExpectedVersion;
pub use crate::api::handler::Handler;
pub use crate::api::query::Query;
pub use crate::api::query::QueryBus;
pub use crate::api::query::QueryError;
pub use crate::api::repository::QueryableRepository;
pub use crate::api::repository::Repository;
pub use crate::api::repository::Spec;
pub use crate::api::service::Service;
pub use crate::api::service::ServiceError;

pub use crate::api::types::{
    ApplicationConfig, DirectCommandBus, DirectQueryBus, EchoHandler, EventBusConfig,
    EventReceiver, HandlerRegistry, InMemoryEventStore, InMemoryRepository, NoopEventBus,
    NoopEventPublisher, OutboundRegistry, Page, RequestContext, RequestContextBuilder,
    ServiceRegistry, TokioEventBus, ValidatorDefault,
};
