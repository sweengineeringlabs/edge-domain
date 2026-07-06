//! Inherent methods for [`PromptTemplate`].

use crate::api::{PromptMetadata, PromptTemplate};

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
        meta.tags = self.category_tags();
        meta
    }

    /// The tag list derived from this template's category.
    fn category_tags(&self) -> Vec<String> {
        vec![self.category.clone()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_identity_fields() {
        let t = PromptTemplate::new("id".into(), "n".into(), "code".into());
        assert_eq!(t.id, "id");
        assert_eq!(t.category, "code");
    }

    /// @covers: metadata
    #[test]
    fn test_metadata_carries_category_as_tag() {
        let t = PromptTemplate::new("id".into(), "n".into(), "code".into());
        assert_eq!(t.metadata().tags, vec!["code".to_string()]);
    }

    /// @covers: category_tags
    #[test]
    fn test_category_tags_wraps_category_in_single_element_vec() {
        let t = PromptTemplate::new("id".into(), "n".into(), "code".into());
        assert_eq!(t.category_tags(), vec!["code".to_string()]);
    }
}
