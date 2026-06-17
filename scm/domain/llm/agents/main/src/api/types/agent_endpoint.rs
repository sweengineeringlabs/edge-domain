//! `AgentEndpoint` — connects the agent primitive to the edge dispatch pipeline.
//!
//! Per ADR-037 this single type carries both faces of a connected native:
//! it implements `Handler` (register the domain + ride the dispatch pipeline)
//! and `Service` (typed, named consumption), with the `Service` face delegating
//! into the `Handler` (Service → Dispatch → Handler → core).

/// Pipeline endpoint for the agent primitive.
///
/// Wraps an agent's skill-execution capability and exposes it as both a
/// dispatchable `Handler` and a typed `Service`. The endpoint is anchored to a
/// single named skill; a request carries the skill input and the response is
/// the skill output.
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
