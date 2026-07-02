use crate::api::provider::types::ModelInfo;

/// Response for [`Provider::model_info`](crate::api::provider::traits::Provider::model_info).
#[derive(Debug, Clone)]
pub struct ModelInfoResponse {
    /// Metadata for the active model.
    pub info: Box<ModelInfo>,
}
