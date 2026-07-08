use crate::api::provider::types::CompletionInput;

/// Request for [`Provider::complete`](crate::api::provider::traits::Provider::complete).
#[derive(Clone, Debug)]
pub struct ProviderCompleteRequest {
    /// Structured completion input to run against this provider's completer.
    pub input: Box<CompletionInput>,
}
