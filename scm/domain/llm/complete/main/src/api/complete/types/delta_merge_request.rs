use crate::api::complete::types::ToolCallDelta;

/// Request for [`ToolOps::merge_delta`](crate::api::complete::traits::ToolOps::merge_delta).
pub struct DeltaMergeRequest<'a> {
    /// Accumulator to merge the incoming fragment into.
    pub existing: &'a mut ToolCallDelta,
    /// Incremental streaming fragment to merge.
    pub incoming: Box<ToolCallDelta>,
}
