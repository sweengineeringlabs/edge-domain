use crate::api::complete::types::CompletionRequest;

/// Request for [`Processor::process`](crate::api::complete::traits::Processor::process).
#[derive(Debug, Clone, Copy)]
pub struct ProcessingRequest<'a> {
    /// The completion request to process.
    pub request: &'a CompletionRequest,
}
