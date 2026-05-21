//! API layer — domain execution-unit contracts.

pub mod application_config_builder;
pub mod command;
pub mod event;
pub mod handler;
pub mod handler_error;
pub mod outbound_registry;
pub mod page;
pub mod query;
pub mod queryable_repository;
pub mod repository;
pub mod repository_error;
pub mod service;
pub mod spec;
pub mod traits;
pub mod validator;

pub use application_config_builder::ApplicationConfigBuilder;
