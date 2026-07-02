use crate::api::provider::types::TokenUsage;

/// Response for [`Provider::last_token_usage`](crate::api::provider::traits::Provider::last_token_usage).
#[derive(Debug, Clone)]
pub struct LastTokenUsageResponse {
    /// Token usage recorded by the most recent completion.
    pub usage: Box<TokenUsage>,
}
