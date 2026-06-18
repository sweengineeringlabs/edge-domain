//! [`EventStore`] impl for [`InMemoryEventStore<E>`] — append-only in-memory store.

use std::time::SystemTime;

use futures::future::BoxFuture;

use crate::api::EventStoreError;
use crate::api::{DomainEvent, EventStore};
use crate::api::{EventEnvelope, ExpectedVersion, InMemoryEventStore};

impl<E> EventStore for InMemoryEventStore<E>
where
    E: DomainEvent + Clone + Send + Sync + 'static,
{
    type Event = E;

    fn append(
        &self,
        aggregate_id: &str,
        events: Vec<Self::Event>,
        expected: ExpectedVersion,
    ) -> BoxFuture<'_, Result<u64, EventStoreError>> {
        let aggregate_id = aggregate_id.to_owned();
        Box::pin(async move {
            let mut streams = self.streams.write();
            let stream = streams.entry(aggregate_id.clone()).or_default();
            let current_version = stream.len() as u64;

            // Optimistic concurrency check
            match expected {
                ExpectedVersion::Any => {}
                ExpectedVersion::NoStream => {
                    if current_version != 0 {
                        return Err(EventStoreError::Conflict {
                            aggregate_id,
                            expected: 0,
                            actual: current_version,
                        });
                    }
                }
                ExpectedVersion::Exact(v) => {
                    if current_version != v {
                        return Err(EventStoreError::Conflict {
                            aggregate_id,
                            expected: v,
                            actual: current_version,
                        });
                    }
                }
            }

            let mut next_seq = current_version + 1;
            for event in events {
                stream.push(EventEnvelope {
                    aggregate_id: aggregate_id.clone(),
                    sequence: next_seq,
                    occurred_at: SystemTime::now(),
                    event,
                });
                next_seq += 1;
            }

            Ok(next_seq - 1)
        })
    }

    fn load(
        &self,
        aggregate_id: &str,
    ) -> BoxFuture<'_, Result<Vec<EventEnvelope<Self::Event>>, EventStoreError>> {
        let aggregate_id = aggregate_id.to_owned();
        Box::pin(async move {
            let streams = self.streams.read();
            Ok(streams
                .get(&aggregate_id)
                .cloned()
                .unwrap_or_default())
        })
    }

    fn load_from(
        &self,
        aggregate_id: &str,
        from_sequence: u64,
    ) -> BoxFuture<'_, Result<Vec<EventEnvelope<Self::Event>>, EventStoreError>> {
        let aggregate_id = aggregate_id.to_owned();
        Box::pin(async move {
            let streams = self.streams.read();
            Ok(streams
                .get(&aggregate_id)
                .map(|s| {
                    s.iter()
                        .filter(|e| e.sequence >= from_sequence)
                        .cloned()
                        .collect()
                })
                .unwrap_or_default())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct InMemoryEventStoreEvt(String);
    impl DomainEvent for InMemoryEventStoreEvt {
        fn event_type(&self) -> &str { "evt" }
        fn aggregate_id(&self) -> &str { &self.0 }
    }

    /// @covers: append
    #[test]
    fn test_append_no_stream_first_write_returns_sequence_happy() {
        let store: InMemoryEventStore<InMemoryEventStoreEvt> = InMemoryEventStore::new();
        let result = futures::executor::block_on(
            store.append("agg-1", vec![InMemoryEventStoreEvt("agg-1".into())], ExpectedVersion::NoStream),
        );
        assert_eq!(result.expect("append"), 1);
    }

    /// @covers: append
    #[test]
    fn test_append_no_stream_when_stream_exists_returns_conflict_error() {
        let store: InMemoryEventStore<InMemoryEventStoreEvt> = InMemoryEventStore::new();
        futures::executor::block_on(
            store.append("agg-2", vec![InMemoryEventStoreEvt("agg-2".into())], ExpectedVersion::NoStream),
        )
        .expect("first append");
        let result = futures::executor::block_on(
            store.append("agg-2", vec![InMemoryEventStoreEvt("agg-2".into())], ExpectedVersion::NoStream),
        );
        assert!(matches!(result, Err(EventStoreError::Conflict { .. })));
    }

    /// @covers: append
    #[test]
    fn test_append_exact_wrong_version_returns_conflict_edge() {
        let store: InMemoryEventStore<InMemoryEventStoreEvt> = InMemoryEventStore::new();
        let result = futures::executor::block_on(
            store.append("agg-3", vec![InMemoryEventStoreEvt("agg-3".into())], ExpectedVersion::Exact(99)),
        );
        assert!(matches!(result, Err(EventStoreError::Conflict { .. })));
    }
}
