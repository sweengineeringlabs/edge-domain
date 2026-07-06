use crate::api::complete::types::ToolChoice;

/// Response for [`ToolOps::tool_choice`](crate::api::complete::traits::ToolOps::tool_choice).
#[derive(Debug, Clone, PartialEq)]
pub struct ToolChoicePreferenceResponse {
    /// Tool calling preference for this completer.
    pub choice: ToolChoice,
}
