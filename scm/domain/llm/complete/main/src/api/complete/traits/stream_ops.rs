//! `StreamOps` — incremental stream accumulation contract.

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::{StreamChunk, StreamDelta};

/// Processing contract for incremental stream data.
pub trait StreamOps: Send + Sync {
    /// Apply an incremental delta to the running chunk accumulator.
    fn apply_delta(
        &self,
        chunk: &mut StreamChunk,
        delta: &StreamDelta,
    ) -> Result<(), CompleteError>;

    /// Convert a delta into an initial [`StreamChunk`] (associated fn).
    fn into_chunk(id: String, delta: StreamDelta) -> StreamChunk
    where
        Self: Sized,
    {
        StreamChunk::partial(id, delta)
    }
}
