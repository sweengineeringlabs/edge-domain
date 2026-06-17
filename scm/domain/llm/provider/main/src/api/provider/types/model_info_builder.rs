//! `ModelInfoBuilder` — fluent builder for [`ModelInfo`].

use crate::api::provider::types::{ModelFamily, ModelInfo};

/// Fluent builder for [`ModelInfo`].
#[derive(Clone, Debug)]
pub struct ModelInfoBuilder {
    id: String,
    name: String,
    family: ModelFamily,
    context_window: u32,
    supports_vision: bool,
    supports_functions: bool,
    supports_streaming: bool,
    training_cutoff: Option<String>,
}

impl Default for ModelInfoBuilder {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            family: ModelFamily::Other,
            context_window: 0,
            supports_vision: false,
            supports_functions: false,
            supports_streaming: false,
            training_cutoff: None,
        }
    }
}

impl ModelInfoBuilder {
    /// Start a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the model identifier.
    pub fn id(mut self, value: String) -> Self {
        self.id = value;
        self
    }

    /// Set the human-readable name.
    pub fn name(mut self, value: String) -> Self {
        self.name = value;
        self
    }

    /// Set the model family.
    pub fn family(mut self, value: ModelFamily) -> Self {
        self.family = value;
        self
    }

    /// Set the context window size.
    pub fn context_window(mut self, value: u32) -> Self {
        self.context_window = value;
        self
    }

    /// Set vision support.
    pub fn supports_vision(mut self, value: bool) -> Self {
        self.supports_vision = value;
        self
    }

    /// Set function-calling support.
    pub fn supports_functions(mut self, value: bool) -> Self {
        self.supports_functions = value;
        self
    }

    /// Set streaming support.
    pub fn supports_streaming(mut self, value: bool) -> Self {
        self.supports_streaming = value;
        self
    }

    /// Set the training-data cutoff.
    pub fn training_cutoff(mut self, value: String) -> Self {
        self.training_cutoff = Some(value);
        self
    }

    /// Build the [`ModelInfo`].
    pub fn build(self) -> ModelInfo {
        let mut info = ModelInfo::new(self.id, self.name, self.family, self.context_window);
        info.supports_vision = self.supports_vision;
        info.supports_functions = self.supports_functions;
        info.supports_streaming = self.supports_streaming;
        info.training_cutoff = self.training_cutoff;
        info
    }
}
