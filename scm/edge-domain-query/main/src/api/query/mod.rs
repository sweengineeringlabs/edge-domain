//! `Query` theme — read operations that never mutate domain state.

pub mod direct_query_bus;
pub mod errors;
pub mod traits;
pub mod types;

pub use direct_query_bus::DirectQueryBus;
pub use errors::QueryError;
pub use traits::{Query, QueryBus, QueryBusFactory};
pub use types::{NoopQuery, StdQueryBusFactory};
