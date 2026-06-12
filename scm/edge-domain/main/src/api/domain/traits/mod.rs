//! Domain theme — port contracts.

pub mod domain_extension;
pub mod domain_factory;

pub use domain_extension::DomainExtension;
pub use domain_factory::DomainFactory;

pub use crate::api::domain::types::Domain;
pub use crate::api::domain::types::NoopDomainExtension;
pub use crate::api::domain::types::OutboundRegistry;
