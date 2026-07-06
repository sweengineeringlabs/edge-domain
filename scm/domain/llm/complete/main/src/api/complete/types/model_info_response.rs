use crate::api::complete::types::ModelInfo;

/// Response for [`Completer::model_info`](crate::api::complete::traits::Completer::model_info)
/// and [`ModelOps::find_model`](crate::api::complete::traits::ModelOps::find_model).
#[derive(Debug, Clone)]
pub struct ModelInfoResponse {
    /// Metadata for the requested model.
    pub info: Box<ModelInfo>,
}
