//! `TemplateProvider` impl for `InMemoryTemplateProvider`.

use crate::api::InMemoryTemplateProvider;
use crate::api::PromptTemplate;
use crate::api::TemplateProvider;

impl TemplateProvider for InMemoryTemplateProvider {
    fn get_template(&self, id: &str) -> Option<&PromptTemplate> {
        self.templates.get(id)
    }

    fn list_templates(&self) -> Vec<&PromptTemplate> {
        self.templates.values().collect()
    }

    fn list_by_category(&self, category: &str) -> Vec<&PromptTemplate> {
        self.templates
            .values()
            .filter(|template| template.category == category)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn template(id: &str, category: &str) -> PromptTemplate {
        PromptTemplate::new(id.to_string(), id.to_string(), category.to_string())
    }

    #[test]
    fn test_get_template_known_id_returns_template() {
        let provider =
            InMemoryTemplateProvider::with_templates(vec![template("code-review", "code")]);
        let found = provider.get_template("code-review");
        assert!(found.is_some());
        assert_eq!(found.expect("present").id, "code-review");
    }

    #[test]
    fn test_get_template_unknown_id_returns_none() {
        let provider = InMemoryTemplateProvider::new();
        assert!(provider.get_template("missing").is_none());
    }

    #[test]
    fn test_list_templates_returns_all_registered() {
        let provider = InMemoryTemplateProvider::with_templates(vec![
            template("a", "code"),
            template("b", "general"),
        ]);
        assert_eq!(provider.list_templates().len(), 2);
    }

    #[test]
    fn test_list_by_category_returns_only_matching_category() {
        let provider = InMemoryTemplateProvider::with_templates(vec![
            template("a", "code"),
            template("b", "general"),
            template("c", "code"),
        ]);
        let code = provider.list_by_category("code");
        assert_eq!(code.len(), 2);
        assert!(code.iter().all(|t| t.category == "code"));
    }

    #[test]
    fn test_list_by_category_unknown_category_is_empty() {
        let provider = InMemoryTemplateProvider::with_templates(vec![template("a", "code")]);
        assert!(provider.list_by_category("nope").is_empty());
    }
}
