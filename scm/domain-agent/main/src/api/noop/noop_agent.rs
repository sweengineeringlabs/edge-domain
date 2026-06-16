//! No-op agent reference marker.

/// Marker type for no-op agent implementations.
///
/// # Examples
///
/// ```ignore
/// let _marker = NoopAgent;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NoopAgent;
