//! `EventStore` trait — append-only event stream persistence contract.

use futures::future::BoxFuture;

use crate::api::event::errors::EventStoreError;
use crate::api::event::traits::DomainEvent;
use crate::api::event::types::{
    EventStoreAppendRequest, EventStoreAppendResponse, EventStoreLoadFromRequest,
    EventStoreLoadFromResponse, EventStoreLoadRequest, EventStoreLoadResponse,
};

/// Append-only storage for domain event streams keyed by aggregate ID.
///
/// Optimistic concurrency is enforced via [`ExpectedVersion`](super::super::types::ExpectedVersion):
/// callers declare what version they read before appending so conflicting
/// writes are detected and rejected.
pub trait EventStore: Send + Sync {
    /// The domain event type stored in this store.
    type Event: DomainEvent + Send + 'static;

    /// Append events to the stream for the given aggregate ID.
    ///
    /// The expected version is checked before writing; a mismatch yields
    /// [`EventStoreError::Conflict`].
    fn append(
        &self,
        req: EventStoreAppendRequest<'_, Self::Event>,
    ) -> BoxFuture<'_, Result<EventStoreAppendResponse, EventStoreError>>;

    /// Load all events for an aggregate in sequence order.
    fn load(
        &self,
        req: EventStoreLoadRequest<'_>,
    ) -> BoxFuture<'_, Result<EventStoreLoadResponse<Self::Event>, EventStoreError>>;

    /// Load events for an aggregate starting at a given sequence number (inclusive).
    fn load_from(
        &self,
        req: EventStoreLoadFromRequest<'_>,
    ) -> BoxFuture<'_, Result<EventStoreLoadFromResponse<Self::Event>, EventStoreError>>;
}
