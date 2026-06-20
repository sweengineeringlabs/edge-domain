//! `Query` theme — read operations that never mutate domain state.

pub mod direct_query_bus;
pub mod errors;
pub mod logging_query_bus;
pub mod traits;
pub mod types;

pub use direct_query_bus::DirectQueryBus;
pub use errors::QueryError;
pub use logging_query_bus::LoggingQueryBus;
pub use traits::{Query, QueryBus, QueryBusBootstrap};
pub use types::{NoopQuery, NoopQueryBus, StdQueryBusFactory};
