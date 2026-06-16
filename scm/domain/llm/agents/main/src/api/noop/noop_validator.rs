//! No-op validator reference marker.

/// Marker type for no-op validator implementations.
///
/// # Examples
///
/// ```ignore
/// let _marker = NoopValidator;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NoopValidator;
