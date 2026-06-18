//! SAF — query service facade.
#[cfg(not(feature = "query"))]
pub use crate::api::DirectQueryBus;
#[cfg(not(feature = "query"))]
pub use crate::api::Query;
#[cfg(not(feature = "query"))]
pub use crate::api::QueryBus;
#[cfg(not(feature = "query"))]
pub use crate::api::QueryError;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const QUERY_SVC: () = ();
