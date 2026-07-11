//! `InMemoryOutboundRegistry` — thread-safe in-memory [`OutboundRegistry`](crate::api::domain::traits::OutboundRegistry).

use std::collections::HashMap;

use parking_lot::RwLock;

/// Thread-safe in-memory registry of outbound handles keyed by name.
///
/// ## Example
///
/// ```rust
/// use edge_domain::{InMemoryOutboundRegistry, OutboundRegisterRequest, OutboundRegistry, OutboundGetRequest};
///
/// let reg: InMemoryOutboundRegistry<String> = InMemoryOutboundRegistry::new();
/// reg.register(OutboundRegisterRequest { name: "anthropic".into(), handle: "https://api.anthropic.com".to_string() }).unwrap();
/// let handle = reg.get(OutboundGetRequest { name: "anthropic".into() }).unwrap().handle;
/// assert_eq!(handle.as_deref(), Some("https://api.anthropic.com"));
/// ```
pub struct InMemoryOutboundRegistry<H: Clone + Send + Sync> {
    pub(crate) handles: RwLock<HashMap<String, H>>,
}
