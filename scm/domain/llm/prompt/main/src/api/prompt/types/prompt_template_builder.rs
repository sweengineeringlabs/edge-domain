//! `PromptTemplateBuilder` — fluent builder for [`PromptTemplate`].

use crate::api::prompt::types::{PromptTemplate, Variable};

/// Fluent builder for [`PromptTemplate`].
#[derive(Clone, Debug, Default)]
pub struct PromptTemplateBuilder {
    id: String,
    name: String,
    category: String,
    system_prompt: String,
    user_template: String,
    description: Option<String>,
    variables: Vec<Variable>,
}

impl PromptTemplateBuilder {
    /// Start a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the catalog identifier.
    pub fn id(mut self, value: String) -> Self {
        self.id = value;
        self
    }

    /// Set the human-readable name.
    pub fn name(mut self, value: String) -> Self {
        self.name = value;
        self
    }

    /// Set the catalog category.
    pub fn category(mut self, value: String) -> Self {
        self.category = value;
        self
    }

    /// Set the system-role prompt body.
    pub fn system_prompt(mut self, value: String) -> Self {
        self.system_prompt = value;
        self
    }

    /// Set the user-role template body.
    pub fn user_template(mut self, value: String) -> Self {
        self.user_template = value;
        self
    }

    /// Set the documentation description.
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the declared variables.
    pub fn variables(mut self, value: Vec<Variable>) -> Self {
        self.variables = value;
        self
    }

    /// Build the [`PromptTemplate`].
    pub fn build(self) -> PromptTemplate {
        let mut template = PromptTemplate::new(self.id, self.name, self.category);
        template.system_prompt = self.system_prompt;
        template.user_template = self.user_template;
        template.description = self.description;
        template.variables = self.variables;
        template
    }
}
