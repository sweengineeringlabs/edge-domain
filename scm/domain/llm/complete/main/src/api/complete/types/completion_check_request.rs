use crate::api::complete::types::CompletionRequest;

/// Request for [`CompleteOps::check`](crate::api::complete::traits::CompleteOps::check).
#[derive(Debug, Clone, Copy)]
pub struct CompletionCheckRequest<'a> {
    /// The completion request to validate.
    pub request: &'a CompletionRequest,
}
