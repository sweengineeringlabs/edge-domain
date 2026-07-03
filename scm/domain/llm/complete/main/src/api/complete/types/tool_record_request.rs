use crate::api::complete::types::Message;

/// Request for [`ToolResultBatch::record`](crate::api::complete::traits::ToolResultBatch::record).
#[derive(Debug)]
pub struct ToolRecordRequest {
    /// Branch index this result belongs to.
    pub index: usize,
    /// The tool-result message to record.
    pub message: Box<Message>,
}
