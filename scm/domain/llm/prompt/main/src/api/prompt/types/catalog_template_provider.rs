//! `CatalogTemplateProvider` — reference [`TemplateProvider`] backed by a map.

use std::collections::BTreeMap;

use crate::api::prompt::types::PromptTemplate;

/// Reference template registry backed by an ordered in-memory map.
///
/// Deterministic iteration order (via [`BTreeMap`]) keeps `list_*` output stable
/// across runs.
#[derive(Clone, Debug, Default)]
pub struct CatalogTemplateProvider {
    pub(crate) templates: BTreeMap<String, PromptTemplate>,
}
