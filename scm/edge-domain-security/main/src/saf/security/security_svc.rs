pub use crate::api::security::NoopSecurity;
pub use crate::api::security::Security;
pub use crate::api::security::SecurityContext;
pub use crate::api::security::SecurityContextBuilder;
pub use crate::api::security::SecurityError;

/// The noop guard singleton — a ready-made [`NoopSecurity`] instance.
pub const NOOP_SECURITY: NoopSecurity = NoopSecurity;
