//! `VariableBuilder` — fluent builder for [`Variable`].

use crate::api::prompt::types::{JsonValue, VariableKind};

/// Fluent builder for [`Variable`](crate::api::prompt::types::Variable).
#[derive(Clone, Debug)]
pub struct VariableBuilder {
    pub(crate) name: String,
    pub(crate) var_type: VariableKind,
    pub(crate) value: Option<JsonValue>,
    pub(crate) default: Option<JsonValue>,
    pub(crate) required: bool,
    pub(crate) description: Option<String>,
}
