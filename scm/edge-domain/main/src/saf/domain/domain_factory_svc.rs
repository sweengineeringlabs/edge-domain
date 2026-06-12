//! SAF — domain factory service facade.
pub use crate::api::domain::traits::DomainFactory;
pub use crate::api::domain::types::Domain;
pub use crate::api::domain::types::NoopDomainExtension;
pub use crate::api::domain::types::OutboundRegistry;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const DOMAIN_FACTORY_SVC: () = ();
