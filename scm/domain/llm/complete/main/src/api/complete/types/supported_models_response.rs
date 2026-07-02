/// Response for [`Completer::supported_models`](crate::api::complete::traits::Completer::supported_models).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedModelsResponse {
    /// Model IDs this completer can serve.
    pub models: Vec<String>,
}
