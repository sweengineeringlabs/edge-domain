//! No-op agent registry reference marker.

/// Marker type for no-op agent registry implementations.
///
/// # Examples
///
/// ```ignore
/// let _marker = NoopAgentRegistry;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NoopAgentRegistry;
