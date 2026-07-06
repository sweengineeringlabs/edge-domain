//! `OutboundRegistry` — thread-safe registry of outbound handles keyed by name.

use std::collections::HashMap;

use parking_lot::RwLock;

/// Thread-safe registry of outbound handles keyed by name.
///
/// ## Example
///
/// ```rust
/// use edge_domain::OutboundRegistry;
///
/// let reg: OutboundRegistry<String> = OutboundRegistry::new();
/// reg.register("anthropic", "https://api.anthropic.com".to_string());
/// assert_eq!(reg.get("anthropic").as_deref(), Some("https://api.anthropic.com"));
/// ```
pub struct OutboundRegistry<H: Clone + Send + Sync> {
    pub(crate) handles: RwLock<HashMap<String, H>>,
}
