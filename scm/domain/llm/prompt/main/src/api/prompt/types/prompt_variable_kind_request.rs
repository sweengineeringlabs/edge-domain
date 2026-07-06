//! [`PromptVariableKindRequest`] — request for [`Prompt::variable_kind`](crate::api::prompt::traits::Prompt::variable_kind).

/// Request for the declared type of the named template variable.
#[derive(Debug, PartialEq)]
pub struct PromptVariableKindRequest<'a> {
    /// The variable name to look up.
    pub name: &'a str,
}
