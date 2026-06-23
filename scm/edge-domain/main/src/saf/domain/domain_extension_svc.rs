//! SAF — domain extension service facade.
pub use crate::api::DomainError;
pub use crate::api::DomainExtension;
pub use crate::api::NoopDomainExtension;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const DOMAIN_EXTENSION_SVC: () = ();
