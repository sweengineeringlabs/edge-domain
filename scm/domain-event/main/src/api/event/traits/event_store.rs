//! `EventStore` trait — append-only event stream persistence contract.

use futures::future::BoxFuture;

use crate::api::event::errors::EventStoreError;
use crate::api::event::traits::DomainEvent;
use crate::api::event::types::{EventEnvelope, ExpectedVersion};

/// Append-only storage for domain event streams keyed by aggregate ID.
///
/// Optimistic concurrency is enforced via [`ExpectedVersion`]: callers
/// declare what version they read before appending so conflicting writes
/// are detected and rejected.
pub trait EventStore: Send + Sync {
    /// The domain event type stored in this store.
    type Event: DomainEvent + Send + 'static;

    /// Append `events` to the stream for `aggregate_id`.
    ///
    /// `expected` is checked before writing; a mismatch yields
    /// [`EventStoreError::Conflict`].
    fn append(
        &self,
        aggregate_id: &str,
        events: Vec<Self::Event>,
        expected: ExpectedVersion,
    ) -> BoxFuture<'_, Result<u64, EventStoreError>>;

    /// Load all events for `aggregate_id` in sequence order.
    fn load(
        &self,
        aggregate_id: &str,
    ) -> BoxFuture<'_, Result<Vec<EventEnvelope<Self::Event>>, EventStoreError>>;

    /// Load events for `aggregate_id` starting at `from_sequence` (inclusive).
    fn load_from(
        &self,
        aggregate_id: &str,
        from_sequence: u64,
    ) -> BoxFuture<'_, Result<Vec<EventEnvelope<Self::Event>>, EventStoreError>>;
}
