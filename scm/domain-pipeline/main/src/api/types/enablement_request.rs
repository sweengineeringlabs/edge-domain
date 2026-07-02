//! [`EnablementRequest`] — zero-sized marker for [`Validator::is_enabled`](crate::Validator::is_enabled).

/// Marker request for checking whether a validator is enabled.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EnablementRequest;
