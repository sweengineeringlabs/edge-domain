//! `PromptMetadataBuilder` — fluent builder for [`PromptMetadata`].

use crate::api::prompt::types::Variable;

/// Fluent builder for [`PromptMetadata`](crate::api::prompt::types::PromptMetadata).
#[derive(Clone, Debug, Default)]
pub struct PromptMetadataBuilder {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) variables: Vec<Variable>,
    pub(crate) description: Option<String>,
    pub(crate) base_token_count: u32,
    pub(crate) tags: Vec<String>,
}
