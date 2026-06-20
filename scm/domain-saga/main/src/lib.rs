//! # edge-domain-saga
//!
//! The `Saga` port contract — long-running process manager driven by events and commands.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use edge_domain_command::Command;
pub use edge_domain_command::CommandError;
pub use edge_domain_event::DomainEvent;
pub use saf::InMemorySagaStore;
pub use saf::NoopSaga;
pub use saf::NoopSagaCommand;
pub use saf::NoopSagaEvent;
pub use saf::Saga;
pub use saf::SagaError;
pub use saf::SagaBootstrap;
pub use saf::SagaStore;
pub use saf::StdSagaFactory;
pub use saf::SAGA_FACTORY_SVC;
pub use saf::SAGA_STORE_SVC;
pub use saf::SAGA_SVC;
