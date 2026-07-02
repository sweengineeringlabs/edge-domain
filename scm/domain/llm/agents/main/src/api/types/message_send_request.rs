use crate::api::types::Message;

/// Request for [`Agent::send`](crate::api::traits::Agent::send).
#[derive(Debug, Clone)]
pub struct MessageSendRequest {
    /// The conversation message to append.
    pub message: Box<Message>,
}
