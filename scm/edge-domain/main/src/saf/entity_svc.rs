//! SAF — entity service facade.
#[cfg(not(feature = "entity"))]
pub use crate::api::Entity;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const ENTITY_SVC: () = ();
