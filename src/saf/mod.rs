//! SAF layer — domain public facade.

mod factory;

pub use factory::{
    direct_command_bus, direct_query_bus, in_memory_repository,
    new_handler_registry, new_service_registry, noop_event_publisher,
};

pub use crate::api::command::Command;
pub use crate::api::command_bus::CommandBus;
pub use crate::api::command_error::CommandError;
pub use crate::api::event::DomainEvent;
pub use crate::api::event_error::EventError;
pub use crate::api::event_publisher::EventPublisher;
pub use crate::api::handler::Handler;
pub use crate::api::handler::{RequestContext, RequestContextBuilder};
pub use crate::api::handler_error::HandlerError;
pub use crate::api::handler::handler_registry::HandlerRegistry;
pub use crate::api::outbound_registry::OutboundRegistry;
pub use crate::api::query::Query;
pub use crate::api::query_bus::QueryBus;
pub use crate::api::query_error::QueryError;
pub use crate::api::repository::Repository;
pub use crate::api::repository_error::RepositoryError;
pub use crate::api::service::Service;
pub use crate::api::service_error::ServiceError;
pub use crate::api::service_registry::ServiceRegistry;
