//! Domain theme — cross-cutting types and value objects.

pub mod domain;
pub mod domain_bootstrap_name_request;
pub mod domain_bootstrap_name_response;
pub mod domain_extension_health_request;
pub mod noop_domain_extension;
pub mod outbound_registry;

pub use domain::Domain;
pub use domain_bootstrap_name_request::DomainBootstrapNameRequest;
pub use domain_bootstrap_name_response::DomainBootstrapNameResponse;
pub use domain_extension_health_request::DomainExtensionHealthRequest;
pub use noop_domain_extension::NoopDomainExtension;
pub use outbound_registry::OutboundRegistry;
