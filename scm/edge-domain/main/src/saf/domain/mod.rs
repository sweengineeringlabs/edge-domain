//! SAF — domain domain facades.
mod domain_bootstrap_svc;
mod domain_event_svc;
mod domain_extension_svc;
mod domain_spi_svc;
mod domain_svc;
pub use self::domain_bootstrap_svc::*;
pub use self::domain_event_svc::*;
pub use self::domain_extension_svc::*;
pub use self::domain_spi_svc::*;
pub use self::domain_svc::*;

// Explicit re-export of Domain for public API
pub use crate::api::Domain;
