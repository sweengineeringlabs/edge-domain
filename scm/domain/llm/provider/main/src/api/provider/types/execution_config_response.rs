use crate::api::provider::types::ExecutionConfig;

/// Response for [`ExecutionModel::config`](crate::api::provider::traits::ExecutionModel::config).
#[derive(Debug, Clone)]
pub struct ExecutionConfigResponse {
    /// Execution configuration (timeouts, token caps, mode).
    pub config: Box<ExecutionConfig>,
}
