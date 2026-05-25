//! In-memory event store — for development and testing only.
//!
//! Backed by a `RwLock<HashMap<String, Vec<EventEnvelope<E>>>>`.
//! State is lost when the process stops.

use std::collections::HashMap;

use parking_lot::RwLock;

use futures::future::BoxFuture;

use crate::api::event::event_store::EventStore;
use crate::api::error::EventStoreError;
use crate::api::event::expected_version::ExpectedVersion;
use crate::api::types::EventEnvelope;
use crate::api::event::DomainEvent;

pub(crate) struct InMemoryEventStore<E> {
    streams: RwLock<HashMap<String, Vec<EventEnvelope<E>>>>,
}

impl<E> InMemoryEventStore<E> {
    pub(crate) fn new() -> Self {
        Self {
            streams: RwLock::new(HashMap::new()),
        }
    }
}

impl<E> EventStore<E> for InMemoryEventStore<E>
where
    E: DomainEvent + Send + Sync + Clone + 'static,
{
    fn append(
        &self,
        aggregate_id: &str,
        events: Vec<E>,
        expected: ExpectedVersion,
    ) -> BoxFuture<'_, Result<u64, EventStoreError>> {
        let aggregate_id = aggregate_id.to_string();
        Box::pin(async move {
            let mut streams = self.streams.write();
            let stream = streams.entry(aggregate_id.clone()).or_default();
            let current_ver = stream.len() as u64;

            match &expected {
                ExpectedVersion::NoStream if current_ver > 0 => {
                    return Err(EventStoreError::Conflict {
                        aggregate_id,
                        expected: 0,
                        actual: current_ver,
                    });
                }
                ExpectedVersion::Exact(v) if *v != current_ver => {
                    return Err(EventStoreError::Conflict {
                        aggregate_id,
                        expected: *v,
                        actual: current_ver,
                    });
                }
                _ => {}
            }

            let start_seq = current_ver + 1;
            for (i, event) in events.into_iter().enumerate() {
                let occurred_at = event.occurred_at();
                stream.push(EventEnvelope {
                    aggregate_id: aggregate_id.clone(),
                    sequence: start_seq + i as u64,
                    occurred_at,
                    event,
                });
            }

            Ok(stream.len() as u64)
        })
    }

    fn load(
        &self,
        aggregate_id: &str,
    ) -> BoxFuture<'_, Result<Vec<EventEnvelope<E>>, EventStoreError>> {
        let aggregate_id = aggregate_id.to_string();
        Box::pin(async move {
            let streams = self.streams.read();
            Ok(streams.get(&aggregate_id).cloned().unwrap_or_default())
        })
    }

    fn load_from(
        &self,
        aggregate_id: &str,
        from_sequence: u64,
    ) -> BoxFuture<'_, Result<Vec<EventEnvelope<E>>, EventStoreError>> {
        let aggregate_id = aggregate_id.to_string();
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
    use std::time::SystemTime;

    #[derive(Clone)]
    struct InMemoryEventStoreTestEvt {
        id: String,
    }
    impl DomainEvent for InMemoryEventStoreTestEvt {
        fn event_type(&self) -> &str {
            "test.evt"
        }
        fn aggregate_id(&self) -> &str {
            &self.id
        }
        fn occurred_at(&self) -> SystemTime {
            SystemTime::now()
        }
    }

    #[test]
    fn test_new_creates_empty_store() {
        let store = InMemoryEventStore::<InMemoryEventStoreTestEvt>::new();
        drop(store);
    }

    #[tokio::test]
    async fn test_append_and_load_roundtrip() {
        let store = InMemoryEventStore::new();
        store
            .append(
                "a1",
                vec![InMemoryEventStoreTestEvt { id: "a1".into() }],
                ExpectedVersion::NoStream,
            )
            .await
            .unwrap();
        let events = store.load("a1").await.unwrap();
        assert_eq!(events.len(), 1);
    }
}
