//! `PromptTemplate` — a named, categorised prompt with a system/user split.

use serde::{Deserialize, Serialize};

use crate::api::prompt::types::{PromptMetadata, Variable};

/// A named prompt template carrying a system/user split and declared variables.
///
/// Richer than [`PromptMetadata`](crate::api::prompt::types::PromptMetadata): it
/// holds the renderable bodies (`system_prompt` + `user_template`) and a
/// `category` for catalog grouping, and can derive a flat `PromptMetadata` via
/// [`metadata`](PromptTemplate::metadata).
///
/// Verbosity variants (per ADR-034 §B) are a deferred enrichment — the catalog
/// contract ([`TemplateProvider`](crate::api::prompt::traits::TemplateProvider))
/// does not depend on them.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PromptTemplate {
    /// Unique identifier (catalog key).
    pub id: String,

    /// Human-readable name.
    pub name: String,

    /// Category for catalog grouping (e.g. `"code"`, `"general"`).
    pub category: String,

    /// System-role prompt body.
    pub system_prompt: String,

    /// User-role template body (with `{{var}}` placeholders).
    pub user_template: String,

    /// Optional documentation.
    pub description: Option<String>,

    /// Variables the `user_template` declares.
    pub variables: Vec<Variable>,
}

impl PromptTemplate {
    /// Construct a template with its required identity fields.
    pub fn new(id: String, name: String, category: String) -> Self {
        Self {
            id,
            name,
            category,
            system_prompt: String::new(),
            user_template: String::new(),
            description: None,
            variables: Vec::new(),
        }
    }

    /// Derive flat [`PromptMetadata`] from this template (category becomes a tag).
    pub fn metadata(&self) -> PromptMetadata {
        let mut meta = PromptMetadata::new(
            self.id.clone(),
            self.name.clone(),
            String::new(),
            self.variables.clone(),
        );
        meta.description = self.description.clone();
        meta.tags = vec![self.category.clone()];
        meta
    }
}
