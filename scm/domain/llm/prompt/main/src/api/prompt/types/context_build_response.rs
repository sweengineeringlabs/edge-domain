//! [`ContextBuildResponse`] — response for [`ContextManager::build_context`](crate::api::prompt::traits::ContextManager::build_context).

use std::collections::HashMap;

use crate::api::prompt::types::JsonValue;

/// The render context built from registered, satisfied variables, flattened
/// from `RenderContext` (SEA `field_type_purity`: a freshly-computed value
/// with no borrow source cannot be nested by reference).
#[derive(Debug, PartialEq)]
pub struct ContextBuildResponse {
    /// Variable name -> value mapping.
    pub variables: HashMap<String, JsonValue>,
    /// Optional metadata (e.g., user_id, session_id).
    pub metadata: HashMap<String, String>,
    /// Template ID being rendered.
    pub template_id: Option<String>,
}
