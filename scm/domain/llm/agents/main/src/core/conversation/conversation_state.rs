use crate::api::Message;

/// Mutable conversation state threaded through a [`ConversationLoop`](crate::api::traits::ConversationLoop) run.
///
/// This is the `Ctx` for the `edge-pipeline` `Pipeline` that drives the loop — pure
/// internal plumbing, never surfaced through any trait signature.
#[derive(Debug, Clone)]
pub(super) struct ConversationState {
    /// Conversation history so far (grows by one or two messages per turn).
    pub(super) messages: Vec<Message>,
    /// Number of turns actually executed (excludes no-op turns past termination).
    pub(super) turns_taken: u32,
    /// Set once a turn produces a terminal finish reason (no more tool/skill calls requested).
    /// Remaining pipeline steps become no-ops once this is `true`.
    pub(super) terminated: bool,
}

impl ConversationState {
    /// Start a fresh (untermined, zero-turn) state from the given initial history.
    pub(super) fn new(messages: Vec<Message>) -> Self {
        Self {
            messages,
            turns_taken: 0,
            terminated: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_state_starts_untermined_with_zero_turns_happy() {
        let state = ConversationState::new(vec![Message::user("hi")]);
        assert_eq!(state.turns_taken, 0);
        assert!(!state.terminated);
        assert_eq!(state.messages.len(), 1);
    }

    /// @covers: new
    #[test]
    fn test_new_empty_history_edge() {
        let state = ConversationState::new(vec![]);
        assert!(state.messages.is_empty());
        assert_eq!(state.turns_taken, 0);
    }

    /// @covers: new
    #[test]
    fn test_new_clone_preserves_all_fields_edge() {
        let mut state = ConversationState::new(vec![Message::user("hi")]);
        state.messages.push(Message::assistant("hello"));
        state.turns_taken = 2;
        state.terminated = true;

        let cloned = state.clone();
        assert_eq!(cloned.messages.len(), state.messages.len());
        assert_eq!(cloned.turns_taken, state.turns_taken);
        assert_eq!(cloned.terminated, state.terminated);
    }
}
