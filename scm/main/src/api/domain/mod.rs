//! Domain theme — cross-cutting domain types and the extension hook contract.

pub mod traits;
pub mod types;

pub use traits::{Domain, DomainExtension, NoopDomainExtension, OutboundRegistry};
