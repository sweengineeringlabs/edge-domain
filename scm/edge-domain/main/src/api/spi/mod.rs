//! SPI layer — domain assembly extension hooks.

mod noop_domain_assembly_hook;
pub mod traits;

pub use noop_domain_assembly_hook::NoopDomainAssemblyHook;
pub use traits::DomainAssemblyHook;
