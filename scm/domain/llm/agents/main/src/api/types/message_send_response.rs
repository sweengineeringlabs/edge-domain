/// Response for [`Agent::send`](crate::api::traits::Agent::send).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageSendResponse {
    /// Running count of messages accepted by this call.
    pub delivered: usize,
}
