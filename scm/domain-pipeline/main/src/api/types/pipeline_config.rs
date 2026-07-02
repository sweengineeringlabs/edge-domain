//! [`PipelineConfig`] — configuration for pipeline execution.

use std::time::Duration;

/// Configuration for pipeline execution behavior.
///
/// ## TOML section `[pipeline]`
/// ```toml
/// [pipeline]
/// timeout_per_step_ms   = 5000   # omit to disable per-step timeout
/// emit_lifecycle_events = false  # default
/// abort_on_error        = true   # default
/// ```
///
/// TOML deserialization is implemented in `core/types/pipeline_config.rs` — this struct
/// carries no `serde` derive so that `api/` never references `serde`'s `Deserializer` type
/// directly (see the `no_foreign_type` SEA rule).
#[derive(Clone, Debug)]
pub struct PipelineConfig {
    /// Per-step execution timeout.
    pub timeout_per_step: Option<Duration>,

    /// Emit lifecycle events when `true`; requires an event bus attached via [`PipelineBuilder::with_event_bus`].
    pub emit_lifecycle_events: bool,

    /// Halt on the first step error when `true`; continue past errors when `false`.
    pub abort_on_error: bool,
}
