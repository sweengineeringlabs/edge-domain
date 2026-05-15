//! API layer — domain execution-unit contracts.

pub mod command;
pub mod command_bus;
pub mod command_error;
pub mod event;
pub mod event_error;
pub mod event_publisher;
pub mod handler;
pub mod handler_error;
pub mod outbound_registry;
pub mod query;
pub mod query_bus;
pub mod repository;
pub mod repository_error;
pub mod service;
pub mod service_error;
pub mod service_registry;
pub mod traits;
pub mod validator;
