use crate::api::reasoning::types::ThinkingProcess;

/// Response for [`Reasoning::reason`](crate::api::reasoning::traits::Reasoning::reason).
#[derive(Debug, Clone)]
pub struct ReasonResponse {
    /// Completed reasoning process.
    pub process: Box<ThinkingProcess>,
}
