//! [`PromptVariableKindResponse`] — response for [`Prompt::variable_kind`](crate::api::prompt::traits::Prompt::variable_kind).

use crate::api::prompt::types::VariableKind;

/// The declared type of the requested variable, if the template declares it.
#[derive(Debug, PartialEq)]
pub struct PromptVariableKindResponse {
    /// The variable's declared kind, if declared.
    pub kind: Option<VariableKind>,
}
