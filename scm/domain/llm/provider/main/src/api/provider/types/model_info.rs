use crate::api::provider::types::ModelFamily;
use serde::{Deserialize, Serialize};
use swe_edge_configbuilder::ConfigSection;

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

impl ModelInfo {
    /// Create a new model info
    pub fn new(id: String, name: String, family: ModelFamily, context_window: u32) -> Self {
        Self {
            id,
            name,
            family,
            context_window,
            supports_vision: false,
            supports_functions: false,
            supports_streaming: false,
            training_cutoff: None,
        }
    }
}

impl ConfigSection for ModelInfo {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies — TOML section key for this type
        "llm.model"
    }
}
