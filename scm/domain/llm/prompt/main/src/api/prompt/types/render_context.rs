use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Context for rendering a template (variable values and metadata)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RenderContext {
    /// Variable name -> value mapping
    pub variables: HashMap<String, serde_json::Value>,

    /// Optional metadata (e.g., user_id, session_id)
    pub metadata: HashMap<String, String>,

    /// Template ID being rendered
    pub template_id: Option<String>,
}

impl RenderContext {
    /// Create a new empty render context
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a variable to context
    pub fn with_variable(mut self, name: String, value: serde_json::Value) -> Self {
        self.variables.insert(name, value);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Set template ID
    pub fn with_template_id(mut self, id: String) -> Self {
        self.template_id = Some(id);
        self
    }

    /// Get a variable value
    pub fn get_variable(&self, name: &str) -> Option<&serde_json::Value> {
        self.variables.get(name)
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|v| v.as_str())
    }

    /// Check if all required variables are present
    pub fn has_all_variables(&self, required: &[&str]) -> bool {
        required
            .iter()
            .all(|name| self.variables.contains_key(*name))
    }

    /// Get count of set variables
    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }
}
