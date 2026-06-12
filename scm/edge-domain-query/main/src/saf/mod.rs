//! SAF — query service facade.

mod query;

pub use crate::api::query::DirectQueryBus;
pub use crate::api::query::Query;
pub use crate::api::query::QueryBus;
pub use crate::api::query::QueryBusFactory;
pub use crate::api::query::QueryError;
