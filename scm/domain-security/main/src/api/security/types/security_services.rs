//! [`SecurityServices`] — zero-configuration [`SecurityBootstrap`] implementation.

/// Zero-configuration implementation of [`SecurityBootstrap`](crate::SecurityBootstrap).
///
/// Provides the standard bootstrap methods with no configuration state.
/// Use this as the default bootstrap in tests and simple deployments.
///
/// ```rust
/// use edge_domain_security::{SecurityBootstrap, SecurityServices};
/// let guard = SecurityServices::noop_guard();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecurityServices;
