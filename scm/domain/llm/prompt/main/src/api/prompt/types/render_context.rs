use crate::api::prompt::types::JsonValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Context for rendering a template (variable values and metadata)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RenderContext {
    /// Variable name -> value mapping
    pub variables: HashMap<String, JsonValue>,

    /// Optional metadata (e.g., user_id, session_id)
    pub metadata: HashMap<String, String>,

    /// Template ID being rendered
    pub template_id: Option<String>,
}
