use crate::api::types::{Message, OwnedHandlerContext};

/// Request for [`ConversationLoop::run`](crate::api::traits::ConversationLoop::run).
///
/// `handler_context` is owned (not a borrowed `HandlerContext<'a>`) because the loop's
/// per-turn [`Step`](edge_domain_pipeline::Step) bridge is constructed once and shared
/// across all configured turns via `Arc` — it cannot hold a non-`'static` borrow. A fresh
/// `HandlerContext` is built from it at each `execute_skill` call.
pub struct ConversationRunRequest {
    /// Conversation history to continue from (may be empty for a fresh conversation).
    pub messages: Vec<Message>,
    /// Maximum number of turns to run before giving up.
    pub max_turns: u32,
    /// Owned security/command-bus/observer context for every skill invocation.
    pub handler_context: Box<OwnedHandlerContext>,
}
