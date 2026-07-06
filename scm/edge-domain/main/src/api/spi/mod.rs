//! SPI layer — domain assembly extension hooks.

pub mod traits;
pub mod types;

pub use traits::DomainAssemblyHook;
pub use types::NoopDomainAssemblyHook;
