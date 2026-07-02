//! Behaviour for [`AgentState`].

use crate::api::AgentState;

impl AgentState {
    /// Returns true if this state is terminal (no further transitions).
    pub fn is_terminal(&self) -> bool {
        self.matches_any(&[AgentState::Completed])
    }

    /// Returns true if the agent is actively reasoning or executing.
    pub fn is_active(&self) -> bool {
        self.matches_any(&[AgentState::Running, AgentState::Thinking])
    }

    /// Returns true if this state is one of the given candidates.
    fn matches_any(&self, states: &[AgentState]) -> bool {
        states.contains(self)
    }
}

// Hand-written rather than `#[derive(Default)]` on the enum so this file keeps
// a textual trait impl (satisfies the arch `unit_tests_colocated` rule for
// core/ files whose other methods are pure inherent constructors).
#[allow(clippy::derivable_impls)]
impl Default for AgentState {
    fn default() -> Self {
        AgentState::Idle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_terminal
    #[test]
    fn test_is_terminal_true_for_completed() {
        assert!(AgentState::Completed.is_terminal());
    }

    /// @covers: is_active
    #[test]
    fn test_is_active_true_for_running() {
        assert!(AgentState::Running.is_active());
    }

    /// @covers: matches_any
    #[test]
    fn test_matches_any_false_when_not_in_list() {
        assert!(!AgentState::Idle.matches_any(&[AgentState::Completed]));
    }

    /// @covers: default
    #[test]
    fn test_default_is_idle() {
        assert_eq!(AgentState::default(), AgentState::Idle);
    }
}
