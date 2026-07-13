//! Domain theme — port contracts.

pub mod domain_extension;
pub mod domain_runtime;
pub mod outbound_registry;

pub use domain_extension::DomainExtension;
pub use domain_runtime::DomainRuntime;
pub use outbound_registry::OutboundRegistry;

pub use crate::api::domain::types::Domain;
pub use crate::api::domain::types::DomainExtensionHealthRequest;
pub use crate::api::domain::types::MemoryOutboundRegistry;
pub use crate::api::domain::types::NoopDomainExtension;
