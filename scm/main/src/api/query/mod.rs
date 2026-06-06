//! `Query` theme — read operations that never mutate domain state.

pub mod error;
pub mod traits;
pub mod types;

pub use error::QueryError;
pub use traits::{Query, QueryBus};
pub use types::DirectQueryBus;
