//! `InMemoryTemplateProvider` — reference [`TemplateProvider`] backed by a map.

use std::collections::BTreeMap;

use crate::api::prompt::types::PromptTemplate;

/// Reference template registry backed by an ordered in-memory map.
///
/// Deterministic iteration order (via [`BTreeMap`]) keeps `list_*` output stable
/// across runs.
#[derive(Clone, Debug, Default)]
pub struct InMemoryTemplateProvider {
    pub(crate) templates: BTreeMap<String, PromptTemplate>,
}

impl InMemoryTemplateProvider {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a registry seeded with `templates` (keyed by each template's `id`).
    pub fn with_templates(templates: Vec<PromptTemplate>) -> Self {
        let mut map = BTreeMap::new();
        for template in templates {
            map.insert(template.id.clone(), template);
        }
        Self { templates: map }
    }

    /// Insert or replace a template (keyed by its `id`).
    pub fn insert(&mut self, template: PromptTemplate) {
        self.templates.insert(template.id.clone(), template);
    }

    /// Number of registered templates.
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    /// Whether no templates are registered.
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
}
