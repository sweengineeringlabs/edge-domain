//! `Query` module — read operations that never mutate domain state.

pub mod direct_query_bus;
#[allow(clippy::module_inception)]
pub mod query;
pub mod query_bus;

pub use query::Query;
pub use query_bus::QueryBus;
pub use crate::api::error::QueryError;
