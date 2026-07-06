use crate::api::complete::types::CompletionRequest;

/// Request for [`Completer::complete`](crate::api::complete::traits::Completer::complete).
#[derive(Debug, Clone, Copy)]
pub struct CompleteRequest<'a> {
    /// The completion request to submit.
    pub request: &'a CompletionRequest,
}
