//! `Query` theme — read operations that never mutate domain state.

pub mod direct_query_bus;
pub mod errors;
pub mod traits;
pub mod types;

pub use errors::QueryError;
pub use traits::{DirectQueryBus, Query, QueryBus};
