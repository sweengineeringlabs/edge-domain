use crate::api::complete::types::{StreamChunk, StreamDelta};

/// Request for [`StreamOps::apply_delta`](crate::api::complete::traits::StreamOps::apply_delta).
pub struct DeltaApplicationRequest<'a> {
    /// Chunk accumulator to update in place.
    pub chunk: &'a mut StreamChunk,
    /// Incremental delta to apply.
    pub delta: &'a StreamDelta,
}
