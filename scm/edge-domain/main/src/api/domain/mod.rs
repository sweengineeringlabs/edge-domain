//! Domain theme — cross-cutting domain types and the extension hook contract.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::DomainError;
pub use traits::{
    Domain, DomainExtension, DomainExtensionHealthRequest, NoopDomainExtension, OutboundRegistry,
};
