//! SAF — projection service facade.
#[cfg(not(feature = "projection"))]
pub use crate::api::projection::Projection;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const PROJECTION_SVC: () = ();
