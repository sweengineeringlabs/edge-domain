//! [`NoopSecurity`] — null-object security guard.

/// Reference [`Security`](crate::Security) guard that accepts every context.
///
/// Use in tests and for open routes that require no authentication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoopSecurity;
