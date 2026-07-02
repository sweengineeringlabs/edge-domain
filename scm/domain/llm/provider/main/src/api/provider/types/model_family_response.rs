use crate::api::provider::types::ModelFamily;

/// Response for [`Provider::model_family`](crate::api::provider::traits::Provider::model_family).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelFamilyResponse {
    /// Model family the active model belongs to.
    pub family: ModelFamily,
}
