/// Mirror of [`crate::api::complete::traits::ToolResultBatch`], declared here to satisfy
/// SEA module correspondence with `core/complete/tool_call_batch.rs` — the trait's sole
/// implementor, `ToolCallBatch`, is `pub(super)`-only internal pipeline plumbing with no
/// public `api/` type of its own.
pub type ToolCallBatch = dyn crate::api::complete::traits::ToolResultBatch;
