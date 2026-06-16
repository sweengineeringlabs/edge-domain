//! # edge-domain-query
//!
//! The `Query` port contract — CQRS read-side with QueryBus and DirectQueryBus.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::DirectQueryBus;
pub use saf::LoggingQueryBus;
pub use saf::NoopQuery;
pub use saf::NoopQueryBus;
pub use saf::Query;
pub use saf::QueryBus;
pub use saf::QueryBusFactory;
pub use saf::QueryError;
pub use saf::StdQueryBusFactory;
pub use saf::QUERY_BUS_FACTORY_SVC;
pub use saf::QUERY_BUS_SVC;
pub use saf::QUERY_SVC;
