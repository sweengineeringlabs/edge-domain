/// Marker satisfying SEA module correspondence with `core/complete/tool_call_step.rs`,
/// which bridges tool-call execution into `edge-domain-pipeline`'s foreign `Step`
/// contract — a foreign trait has no home in this crate's own `api/`, hence this marker.
pub struct ToolCallStep;
