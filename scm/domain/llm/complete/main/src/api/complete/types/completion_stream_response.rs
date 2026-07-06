use futures::stream::BoxStream;

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::StreamChunk;

/// Response for [`Completer::complete_stream`](crate::api::complete::traits::Completer::complete_stream).
pub struct CompletionStreamResponse {
    /// Live stream of incremental completion chunks.
    pub stream: BoxStream<'static, Result<StreamChunk, CompleteError>>,
}
