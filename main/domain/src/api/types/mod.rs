//! Cross-theme types for the domain layer — consumed by ≥2 themes or
//! crate-level concerns.

pub mod application_config;
pub mod domain;
pub mod noop_domain_extension;
pub mod outbound_registry;

pub use application_config::ApplicationConfig;
pub use domain::Domain;
pub use noop_domain_extension::NoopDomainExtension;
pub use outbound_registry::OutboundRegistry;
