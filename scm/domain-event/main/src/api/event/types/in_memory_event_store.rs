//! [`InMemoryEventStore<E>`] — in-memory event store for testing and development.

use std::collections::HashMap;

use parking_lot::RwLock;

use crate::api::event::traits::DomainEvent;
use crate::api::event::types::EventEnvelope;

/// An in-memory event store keyed by aggregate ID.
///
/// Uses a [`parking_lot::RwLock`] so reads are fully concurrent.
/// Intended for unit tests and local development; not for production use.
pub struct InMemoryEventStore<E: DomainEvent + Clone + Send + Sync + 'static> {
    pub(crate) streams: RwLock<HashMap<String, Vec<EventEnvelope<E>>>>,
}

impl<E: DomainEvent + Clone + Send + Sync + 'static> InMemoryEventStore<E> {
    /// Create an empty store.
    pub fn new() -> Self {
        Self {
            streams: RwLock::new(HashMap::new()),
        }
    }
}

impl<E: DomainEvent + Clone + Send + Sync + 'static> Default for InMemoryEventStore<E> {
    fn default() -> Self {
        Self::new()
    }
}
