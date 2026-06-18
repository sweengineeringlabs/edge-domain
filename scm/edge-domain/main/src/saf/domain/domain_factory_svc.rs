//! SAF — domain factory service facade.
pub use crate::api::DomainFactory;
pub use crate::api::Domain;
pub use crate::api::NoopDomainExtension;
pub use crate::api::OutboundRegistry;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const DOMAIN_FACTORY_SVC: () = ();
