//! Domain theme — port contracts.

pub mod domain_extension;
pub mod domain_runtime;
pub mod outbound_registry;

pub use domain_extension::DomainExtension;
pub use domain_runtime::DomainRuntime;
pub use outbound_registry::OutboundRegistry;

pub use crate::api::domain::domain::Domain;
pub use crate::api::domain::dto::DomainExtensionHealthRequest;
pub use crate::api::domain::memory_outbound_registry::MemoryOutboundRegistry;
pub use crate::api::domain::noop_domain_extension::NoopDomainExtension;
