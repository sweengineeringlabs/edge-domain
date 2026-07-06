use crate::api::complete::types::ModelInfo;

/// Response for [`Completer::list_models`](crate::api::complete::traits::Completer::list_models).
#[derive(Debug, Clone)]
pub struct ListModelsResponse {
    /// All models available to this completer.
    pub models: Vec<ModelInfo>,
}
