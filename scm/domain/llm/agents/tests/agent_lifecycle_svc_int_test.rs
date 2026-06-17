#![allow(clippy::unwrap_used, clippy::expect_used)]
//! SAF tests for the `AgentLifecycle` trait and `AGENT_LIFECYCLE_SVC` constant.

use std::sync::Mutex;

use edge_llm_agent::{
    AgentLifecycle, AgentLifecycleError, AgentState, NoopAgentLifecycle, AGENT_LIFECYCLE_SVC,
};
use futures::executor::block_on;

/// A stateful lifecycle used to exercise both success and failure paths.
struct StatefulLifecycle {
    state: Mutex<AgentState>,
}

impl StatefulLifecycle {
    fn new(initial: AgentState) -> Self {
        Self {
            state: Mutex::new(initial),
        }
    }
}

#[async_trait::async_trait]
impl AgentLifecycle for StatefulLifecycle {
    fn current_state(&self) -> AgentState {
        *self.state.lock().unwrap()
    }

    async fn transition_to(&self, target: AgentState) -> Result<(), AgentLifecycleError> {
        let mut guard = self.state.lock().unwrap();
        if guard.is_terminal() {
            return Err(AgentLifecycleError::InvalidTransition {
                from: *guard,
                to: target,
            });
        }
        *guard = target;
        Ok(())
    }
}

// --- AGENT_LIFECYCLE_SVC ---

#[test]
fn test_agent_lifecycle_svc_constant_value() {
    assert_eq!(AGENT_LIFECYCLE_SVC, "agent_lifecycle");
}

// --- current_state ---

/// @covers: current_state
#[test]
fn test_current_state_reflects_initial_happy() {
    let lc = StatefulLifecycle::new(AgentState::Running);
    assert_eq!(lc.current_state(), AgentState::Running);
}

/// @covers: current_state
#[test]
fn test_current_state_noop_stays_idle_error() {
    // The no-op lifecycle never leaves Idle even after a rejected transition.
    let lc = NoopAgentLifecycle;
    let _ = block_on(lc.transition_to(AgentState::Running));
    assert_eq!(lc.current_state(), AgentState::Idle);
}

/// @covers: current_state
#[test]
fn test_current_state_after_transition_edge() {
    let lc = StatefulLifecycle::new(AgentState::Idle);
    block_on(lc.transition_to(AgentState::Thinking)).unwrap();
    assert_eq!(lc.current_state(), AgentState::Thinking);
}

// --- is_in ---

/// @covers: is_in
#[test]
fn test_is_in_matching_state_happy() {
    let lc = StatefulLifecycle::new(AgentState::Paused);
    assert!(lc.is_in(AgentState::Paused));
}

/// @covers: is_in
#[test]
fn test_is_in_non_matching_state_error() {
    let lc = StatefulLifecycle::new(AgentState::Paused);
    assert!(!lc.is_in(AgentState::Running));
}

/// @covers: is_in
#[test]
fn test_is_in_terminal_state_edge() {
    let lc = StatefulLifecycle::new(AgentState::Completed);
    assert!(lc.is_in(AgentState::Completed));
}

// --- transition_to ---

/// @covers: transition_to
#[test]
fn test_transition_to_valid_target_happy() {
    let lc = StatefulLifecycle::new(AgentState::Idle);
    assert!(block_on(lc.transition_to(AgentState::Running)).is_ok());
}

/// @covers: transition_to
#[test]
fn test_transition_to_from_terminal_error() {
    let lc = StatefulLifecycle::new(AgentState::Completed);
    let result = block_on(lc.transition_to(AgentState::Running));
    assert!(matches!(
        result,
        Err(AgentLifecycleError::InvalidTransition { .. })
    ));
}

/// @covers: transition_to
#[test]
fn test_transition_to_same_state_edge() {
    let lc = StatefulLifecycle::new(AgentState::Running);
    assert!(block_on(lc.transition_to(AgentState::Running)).is_ok());
}

// --- pause ---

/// @covers: pause
#[test]
fn test_pause_sets_paused_happy() {
    let lc = StatefulLifecycle::new(AgentState::Running);
    block_on(lc.pause()).unwrap();
    assert_eq!(lc.current_state(), AgentState::Paused);
}

/// @covers: pause
#[test]
fn test_pause_from_terminal_error() {
    let lc = StatefulLifecycle::new(AgentState::Completed);
    assert!(block_on(lc.pause()).is_err());
}

/// @covers: pause
#[test]
fn test_pause_when_already_paused_edge() {
    let lc = StatefulLifecycle::new(AgentState::Paused);
    assert!(block_on(lc.pause()).is_ok());
}

// --- resume ---

/// @covers: resume
#[test]
fn test_resume_sets_running_happy() {
    let lc = StatefulLifecycle::new(AgentState::Paused);
    block_on(lc.resume()).unwrap();
    assert_eq!(lc.current_state(), AgentState::Running);
}

/// @covers: resume
#[test]
fn test_resume_from_terminal_error() {
    let lc = StatefulLifecycle::new(AgentState::Completed);
    assert!(block_on(lc.resume()).is_err());
}

/// @covers: resume
#[test]
fn test_resume_when_already_running_edge() {
    let lc = StatefulLifecycle::new(AgentState::Running);
    assert!(block_on(lc.resume()).is_ok());
}

// --- abort ---

/// @covers: abort
#[test]
fn test_abort_sets_idle_happy() {
    let lc = StatefulLifecycle::new(AgentState::Running);
    block_on(lc.abort()).unwrap();
    assert_eq!(lc.current_state(), AgentState::Idle);
}

/// @covers: abort
#[test]
fn test_abort_from_terminal_error() {
    let lc = StatefulLifecycle::new(AgentState::Completed);
    assert!(block_on(lc.abort()).is_err());
}

/// @covers: abort
#[test]
fn test_abort_when_already_idle_edge() {
    let lc = StatefulLifecycle::new(AgentState::Idle);
    assert!(block_on(lc.abort()).is_ok());
}
