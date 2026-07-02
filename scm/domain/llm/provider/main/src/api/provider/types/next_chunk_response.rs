use crate::api::provider::types::StreamChunk;

/// Response for [`StreamHandler::next_chunk`](crate::api::provider::traits::StreamHandler::next_chunk).
#[derive(Debug, Clone)]
pub struct NextChunkResponse {
    /// The next fully-formed chunk, if one was ready.
    pub chunk: Option<Box<StreamChunk>>,
}
