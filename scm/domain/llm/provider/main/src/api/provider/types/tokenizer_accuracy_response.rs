use crate::api::provider::types::TokenizerAccuracy;

/// Response for [`Provider::tokenizer_accuracy`](crate::api::provider::traits::Provider::tokenizer_accuracy).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenizerAccuracyResponse {
    /// Accuracy of this provider's token counting.
    pub accuracy: TokenizerAccuracy,
}
