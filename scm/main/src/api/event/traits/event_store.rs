//! `EventStore` — provider-agnostic contract for appending and loading events.

use futures::future::BoxFuture;

use crate::api::event::DomainEvent;
use crate::api::event::EventEnvelope;
use crate::api::event::EventStoreError;
use crate::api::event::ExpectedVersion;

/// Append-only store for domain events, keyed by aggregate stream ID.
///
/// Implementations are provider-specific (EventStoreDB, PostgreSQL, in-memory, …)
/// and live in infrastructure crates.  The domain layer only depends on this trait.
///
/// ## Optimistic concurrency
///
/// Pass [`ExpectedVersion`] to `append` to guard against lost updates.
/// The store rejects the write with [`EventStoreError::Conflict`] when the
/// actual stream version differs from the expected one.
///
/// ## Example
///
/// ```rust,ignore
/// let store = swe_edge_domain::new_in_memory_event_store::<OrderEvent>();
///
/// store.append("order-1", vec![OrderCreated { … }], ExpectedVersion::NoStream).await?;
///
/// let order = swe_edge_domain::reconstitute::<Order>(&*store, "order-1").await?;
/// ```
pub trait EventStore<E>: Send + Sync
where
    E: DomainEvent + Send + 'static,
{
    /// Append `events` to the stream identified by `aggregate_id`.
    ///
    /// `expected` controls optimistic concurrency — see [`ExpectedVersion`].
    ///
    /// Returns the new stream version (sequence number of the last appended event).
    fn append(
        &self,
        aggregate_id: &str,
        events: Vec<E>,
        expected: ExpectedVersion,
    ) -> BoxFuture<'_, Result<u64, EventStoreError>>;

    /// Load all events for `aggregate_id` in version order.
    ///
    /// Returns an empty `Vec` when the stream does not exist — callers use
    /// this to distinguish "aggregate never created" from an error.
    fn load(
        &self,
        aggregate_id: &str,
    ) -> BoxFuture<'_, Result<Vec<EventEnvelope<E>>, EventStoreError>>;

    /// Load events for `aggregate_id` starting at `from_sequence` (inclusive).
    ///
    /// Used for snapshot-based reconstitution — load a snapshot, then replay
    /// only the events that occurred after it.
    fn load_from(
        &self,
        aggregate_id: &str,
        from_sequence: u64,
    ) -> BoxFuture<'_, Result<Vec<EventEnvelope<E>>, EventStoreError>>;
}
