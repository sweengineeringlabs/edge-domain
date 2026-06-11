//! Domain theme — cross-cutting types and value objects.

pub mod application_config;
pub mod domain;
pub mod outbound_registry;

pub use application_config::ApplicationConfig;
pub use domain::Domain;
pub use outbound_registry::OutboundRegistry;
