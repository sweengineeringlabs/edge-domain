//! `EventEnvelope` — a domain event together with its store metadata.

use std::time::SystemTime;

/// A domain event together with the metadata assigned by the event store.
///
/// Returned by [`EventStore::load`](crate::EventStore::load) and
/// [`EventStore::load_from`](crate::EventStore::load_from).
#[derive(Debug, Clone)]
pub struct EventEnvelope<E> {
    /// Identity of the aggregate stream this event belongs to.
    pub aggregate_id: String,

    /// Monotonically increasing position within the aggregate stream.
    ///
    /// Starts at 1 for the first event.  Used for optimistic concurrency
    /// control via [`ExpectedVersion::Exact`](crate::ExpectedVersion::Exact).
    pub sequence: u64,

    /// Wall-clock time at which the event occurred.
    pub occurred_at: SystemTime,

    /// The domain event payload.
    pub event: E,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_envelope_fields_are_accessible() {
        let env = EventEnvelope {
            aggregate_id: "agg-1".into(),
            sequence: 1,
            occurred_at: SystemTime::UNIX_EPOCH,
            event: "payload",
        };
        assert_eq!(env.aggregate_id, "agg-1");
        assert_eq!(env.sequence, 1);
        assert_eq!(env.event, "payload");
    }

    #[test]
    fn test_event_envelope_clone() {
        let env = EventEnvelope {
            aggregate_id: "agg-1".into(),
            sequence: 2,
            occurred_at: SystemTime::UNIX_EPOCH,
            event: 42u32,
        };
        let cloned = env.clone();
        assert_eq!(cloned.sequence, 2);
        assert_eq!(cloned.event, 42);
    }
}
