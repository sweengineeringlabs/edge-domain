//! No-op agent manager reference marker.

/// Marker type for no-op agent manager implementations.
///
/// # Examples
///
/// ```ignore
/// let _marker = NoopAgentManager;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NoopAgentManager;
