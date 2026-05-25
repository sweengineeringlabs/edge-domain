//! SAF layer — domain public facade.

mod edge_domain_svc;

pub use edge_domain_svc::{
    direct_command_bus, direct_query_bus, echo_handler, new_handler_registry,
    new_in_memory_event_store, new_in_memory_queryable_repository, new_in_memory_repository,
    new_service_registry, noop_event_bus, noop_event_publisher, reconstitute, tokio_event_bus,
    validate_config,
};

pub use crate::api::command::Command;
pub use crate::api::command::CommandBus;
pub use crate::api::command::CommandError;
pub use crate::api::event::Aggregate;
pub use crate::api::event::DomainEvent;
pub use crate::api::event::EventBus;
pub use crate::api::event::EventBusConfig;
pub use crate::api::event::EventEnvelope;
pub use crate::api::event::EventError;
pub use crate::api::event::EventPublisher;
pub use crate::api::event::EventReceiver;
pub use crate::api::event::EventStore;
pub use crate::api::event::EventStoreError;
pub use crate::api::event::ExpectedVersion;
pub use crate::api::handler::handler_registry::HandlerRegistry;
pub use crate::api::handler::EchoHandler;
pub use crate::api::handler::Handler;
pub use crate::api::handler::{RequestContext, RequestContextBuilder};
pub use crate::api::handler_error::HandlerError;
pub use crate::api::outbound_registry::OutboundRegistry;
pub use crate::api::page::Page;
pub use crate::api::query::Query;
pub use crate::api::query::QueryBus;
pub use crate::api::query::QueryError;
pub use crate::api::queryable_repository::QueryableRepository;
pub use crate::api::repository::Repository;
pub use crate::api::repository_error::RepositoryError;
pub use crate::api::service::Service;
pub use crate::api::service::ServiceError;
pub use crate::api::service::ServiceRegistry;
pub use crate::api::spec::Spec;
pub use crate::api::traits::Validator;
