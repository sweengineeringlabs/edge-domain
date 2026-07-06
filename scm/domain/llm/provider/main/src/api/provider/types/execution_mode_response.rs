use crate::api::provider::types::ExecutionMode;

/// Response for [`ExecutionModel::execution_mode`](crate::api::provider::traits::ExecutionModel::execution_mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionModeResponse {
    /// Execution mode this model operates in.
    pub mode: ExecutionMode,
}
