use crate::api::provider::types::ModelFamily;
use serde::{Deserialize, Serialize};

/// LLM model metadata
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model ID (e.g., "gpt-4-turbo")
    pub id: String,

    /// Model name
    pub name: String,

    /// Model provider/family
    pub family: ModelFamily,

    /// Context window size
    pub context_window: u32,

    /// Supports vision/images
    pub supports_vision: bool,

    /// Supports function calling
    pub supports_functions: bool,

    /// Supports streaming
    pub supports_streaming: bool,

    /// Training data cutoff date
    pub training_cutoff: Option<String>,
}
