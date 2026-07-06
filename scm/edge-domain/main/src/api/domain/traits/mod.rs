//! Domain theme — port contracts.

pub mod domain_bootstrap;
pub mod domain_extension;

pub use domain_bootstrap::DomainBootstrap;
pub use domain_extension::DomainExtension;

pub use crate::api::domain::types::Domain;
pub use crate::api::domain::types::DomainBootstrapNameRequest;
pub use crate::api::domain::types::DomainBootstrapNameResponse;
pub use crate::api::domain::types::DomainExtensionHealthRequest;
pub use crate::api::domain::types::NoopDomainExtension;
pub use crate::api::domain::types::OutboundRegistry;
