use crate::api::complete::types::CompletionRequest;

/// Request for [`Validator::validate`](crate::api::complete::traits::Validator::validate).
#[derive(Debug, Clone, Copy)]
pub struct ValidationRequest<'a> {
    /// The completion request to validate.
    pub request: &'a CompletionRequest,
}
