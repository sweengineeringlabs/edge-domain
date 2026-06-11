//! Domain theme — cross-cutting domain types and the extension hook contract.

pub mod traits;
pub mod types;

pub use traits::DomainExtension;
pub use types::ApplicationConfig;
pub use types::Domain;
pub use types::NoopDomainExtension;
pub use types::OutboundRegistry;
