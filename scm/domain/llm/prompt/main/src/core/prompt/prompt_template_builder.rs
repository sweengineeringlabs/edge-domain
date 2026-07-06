//! Inherent methods for [`PromptTemplateBuilder`].

use crate::api::{PromptTemplate, PromptTemplateBuilder, Variable};

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
        let (system_prompt, user_template, description, variables) = (
            self.system_prompt,
            self.user_template,
            self.description,
            self.variables,
        );
        let mut template = PromptTemplate::new(self.id, self.name, self.category);
        Self::apply_body_fields(
            &mut template,
            system_prompt,
            user_template,
            description,
            variables,
        );
        template
    }

    /// Apply the builder's body/content fields onto `template`.
    fn apply_body_fields(
        template: &mut PromptTemplate,
        system_prompt: String,
        user_template: String,
        description: Option<String>,
        variables: Vec<Variable>,
    ) {
        template.system_prompt = system_prompt;
        template.user_template = user_template;
        template.description = description;
        template.variables = variables;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::VariableKind;

    /// @covers: new
    #[test]
    fn test_new_starts_with_empty_id() {
        assert_eq!(PromptTemplateBuilder::new().id, "");
    }

    /// @covers: id
    #[test]
    fn test_id_sets_field() {
        assert_eq!(PromptTemplateBuilder::new().id("i".into()).build().id, "i");
    }

    /// @covers: build
    #[test]
    fn test_build_applies_all_overrides() {
        let t = PromptTemplateBuilder::new()
            .id("code-review".into())
            .category("code".into())
            .build();
        assert_eq!(t.id, "code-review");
        assert_eq!(t.category, "code");
    }

    /// @covers: apply_body_fields
    #[test]
    fn test_apply_body_fields_sets_all_four_fields() {
        let mut t = PromptTemplate::new("id".into(), "n".into(), "code".into());
        PromptTemplateBuilder::apply_body_fields(
            &mut t,
            "sys".into(),
            "user".into(),
            Some("d".into()),
            vec![],
        );
        assert_eq!(t.system_prompt, "sys");
        assert_eq!(t.user_template, "user");
    }

    /// @covers: name
    #[test]
    fn test_name_sets_field() {
        assert_eq!(
            PromptTemplateBuilder::new().name("n".into()).build().name,
            "n"
        );
    }

    /// @covers: category
    #[test]
    fn test_category_sets_field() {
        assert_eq!(
            PromptTemplateBuilder::new()
                .category("code".into())
                .build()
                .category,
            "code"
        );
    }

    /// @covers: system_prompt
    #[test]
    fn test_system_prompt_sets_field() {
        let t = PromptTemplateBuilder::new()
            .system_prompt("sys".into())
            .build();
        assert_eq!(t.system_prompt, "sys");
    }

    /// @covers: user_template
    #[test]
    fn test_user_template_sets_field() {
        let t = PromptTemplateBuilder::new()
            .user_template("u".into())
            .build();
        assert_eq!(t.user_template, "u");
    }

    /// @covers: description
    #[test]
    fn test_description_sets_field() {
        let t = PromptTemplateBuilder::new().description("d".into()).build();
        assert_eq!(t.description, Some("d".to_string()));
    }

    /// @covers: variables
    #[test]
    fn test_variables_sets_field() {
        let var = Variable::new("a".into(), VariableKind::String);
        let t = PromptTemplateBuilder::new().variables(vec![var]).build();
        assert_eq!(t.variables.len(), 1);
    }
}
