//! `Query` theme — read operations that never mutate domain state.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::QueryError;
pub use traits::{DirectQueryBus, Query, QueryBus, QueryBusFactory};
