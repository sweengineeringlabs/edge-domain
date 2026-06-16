//! [`SecurityServices`] — zero-configuration [`SecurityFactory`] implementation.

/// Zero-configuration implementation of [`SecurityFactory`](crate::SecurityFactory).
///
/// Provides the standard factory methods with no configuration state.
/// Use this as the default factory in tests and simple deployments.
///
/// ```rust
/// use edge_domain_security::{SecurityFactory, SecurityServices};
/// let guard = SecurityServices::noop_guard();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityServices;
