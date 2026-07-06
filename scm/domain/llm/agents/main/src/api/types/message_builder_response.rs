use crate::api::types::MessageBuilder;

/// Response for [`Agent::message_builder`](crate::api::traits::Agent::message_builder).
pub struct MessageBuilderResponse {
    /// A fluent builder for composing a message to this agent.
    pub builder: Box<MessageBuilder>,
}
