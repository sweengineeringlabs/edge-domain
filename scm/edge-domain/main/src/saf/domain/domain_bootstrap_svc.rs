//! SAF — domain bootstrap service facade.
pub use crate::api::Domain;
pub use crate::api::DomainBootstrap;
pub use crate::api::NoopDomainExtension;
pub use crate::api::OutboundRegistry;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const DOMAIN_BOOTSTRAP_SVC: () = ();
