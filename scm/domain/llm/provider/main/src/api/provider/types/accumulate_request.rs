use crate::api::provider::types::StreamDelta;

/// Request for [`StreamHandler::accumulate`](crate::api::provider::traits::StreamHandler::accumulate).
#[derive(Debug, Clone)]
pub struct AccumulateRequest {
    /// Incremental delta to fold into the in-progress response.
    pub delta: StreamDelta,
}
