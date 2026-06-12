pub use crate::api::security::SecurityFactory;
pub use crate::api::security::SecurityServices;

/// Default factory constant — a ready-made [`SecurityServices`] instance.
pub const DEFAULT_SERVICES: SecurityServices = SecurityServices;
