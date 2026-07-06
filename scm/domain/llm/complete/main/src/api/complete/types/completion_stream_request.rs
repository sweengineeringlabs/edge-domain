use crate::api::complete::types::CompletionRequest;

/// Request for [`Completer::complete_stream`](crate::api::complete::traits::Completer::complete_stream).
#[derive(Debug, Clone, Copy)]
pub struct CompletionStreamRequest<'a> {
    /// The completion request to submit.
    pub request: &'a CompletionRequest,
}
