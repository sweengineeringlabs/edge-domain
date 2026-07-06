//! Domain theme — cross-cutting domain types and the extension hook contract.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::DomainError;
pub use traits::{
    Domain, DomainBootstrap, DomainBootstrapNameRequest, DomainBootstrapNameResponse,
    DomainExtension, DomainExtensionHealthRequest, NoopDomainExtension, OutboundRegistry,
};
