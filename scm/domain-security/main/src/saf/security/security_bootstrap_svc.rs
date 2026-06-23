//! SAF — security bootstrap service facade.
pub use crate::api::SecurityBootstrap;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const SECURITY_BOOTSTRAP_SVC: () = ();
