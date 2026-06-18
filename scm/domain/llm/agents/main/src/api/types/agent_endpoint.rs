//! `AgentEndpoint` — connects the agent primitive to the edge dispatch pipeline.

/// Pipeline endpoint for the agent primitive.
///
/// Wraps an agent's skill-execution capability and exposes it as a dispatchable
/// `Handler` (ADR-024). The endpoint is anchored to a single named skill;
/// a request carries the skill input and the response is the skill output.
#[derive(Clone, Debug)]
pub struct AgentEndpoint {
    pub(crate) skill: String,
}

impl AgentEndpoint {
    /// Construct an endpoint that routes its requests to the named skill.
    pub fn new(skill: impl Into<String>) -> Self {
        Self {
            skill: skill.into(),
        }
    }

    /// The name of the skill this endpoint dispatches to.
    pub fn skill(&self) -> &str {
        &self.skill
    }
}
