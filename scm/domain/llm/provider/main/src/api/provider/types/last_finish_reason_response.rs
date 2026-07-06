use crate::api::provider::types::FinishReason;

/// Response for [`Provider::last_finish_reason`](crate::api::provider::traits::Provider::last_finish_reason).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LastFinishReasonResponse {
    /// Finish reason recorded by the most recent completion.
    pub reason: FinishReason,
}
