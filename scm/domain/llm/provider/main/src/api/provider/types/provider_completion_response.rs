use edge_llm_complete::CompletionResponse;

/// Response for [`Provider::complete`](crate::api::provider::traits::Provider::complete).
#[derive(Clone, Debug)]
pub struct ProviderCompletionResponse {
    /// The completion result returned by the underlying [`Completer`](edge_llm_complete::Completer).
    pub response: CompletionResponse,
}
