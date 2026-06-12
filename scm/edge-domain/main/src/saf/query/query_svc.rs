//! SAF — query service facade.
#[cfg(not(feature = "query"))]
pub use crate::api::query::DirectQueryBus;
#[cfg(not(feature = "query"))]
pub use crate::api::query::Query;
#[cfg(not(feature = "query"))]
pub use crate::api::query::QueryBus;
#[cfg(not(feature = "query"))]
pub use crate::api::query::QueryError;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const QUERY_SVC: () = ();
