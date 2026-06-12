//! SAF — domain extension service facade.
pub use crate::api::domain::traits::DomainExtension;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const DOMAIN_EXTENSION_SVC: () = ();
