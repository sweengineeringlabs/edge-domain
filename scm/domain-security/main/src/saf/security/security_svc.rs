pub use crate::api::NoopSecurity;
pub use crate::api::Security;
pub use crate::api::SecurityContext;
pub use crate::api::SecurityContextBuilder;
pub use crate::api::SecurityError;

/// The noop guard singleton — a ready-made [`NoopSecurity`] instance.
pub const NOOP_SECURITY: NoopSecurity = NoopSecurity;
