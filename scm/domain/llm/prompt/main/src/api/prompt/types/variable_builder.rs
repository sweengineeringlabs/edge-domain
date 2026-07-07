//! `VariableBuilder` — fluent builder for [`Variable`].

use crate::api::prompt::types::{JsonValue, VariableKind};

/// Fluent builder for [`Variable`](crate::api::prompt::types::Variable).
///
/// Orphan-type note: `ContextManager`'s methods pass `&Variable` by reference
/// (`RegisterVariableRequest`), never this builder — plain builder, no interface behind it,
/// same rationale as `PromptCacheBuilder`.
#[derive(Clone, Debug)]
pub struct VariableBuilder {
    pub(crate) name: String,
    pub(crate) var_type: VariableKind,
    pub(crate) value: Option<JsonValue>,
    pub(crate) default: Option<JsonValue>,
    pub(crate) required: bool,
    pub(crate) description: Option<String>,
}
