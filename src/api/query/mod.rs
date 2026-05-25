//! `Query` module — read operations that never mutate domain state.

#[allow(clippy::module_inception)]
pub mod query;
pub mod query_bus;
pub mod query_error;

pub use query::Query;
pub use query_bus::QueryBus;
pub use query_error::QueryError;
