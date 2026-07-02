use crate::api::types::ToolChoice;

/// Response for [`Agent::tool_choice`](crate::api::traits::Agent::tool_choice).
#[derive(Debug, Clone, PartialEq)]
pub struct ToolChoicePreferenceResponse {
    /// The tool-invocation policy this agent applies to its skills.
    pub choice: ToolChoice,
}
