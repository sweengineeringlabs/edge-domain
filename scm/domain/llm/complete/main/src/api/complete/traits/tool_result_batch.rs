//! `ToolResultBatch` — records tool-execution results by branch index.

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::ToolRecordRequest;

/// Records the completed tool-result message for one branch of a per-turn
/// `ParallelStep` fan-out (see `edge-domain-pipeline`'s `ParallelStep`).
pub trait ToolResultBatch: Send {
    /// Record the completed tool-result message for the branch named in `req`.
    fn record(&self, req: ToolRecordRequest) -> Result<(), CompleteError>;
}
