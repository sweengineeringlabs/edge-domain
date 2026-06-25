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
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct PipelineConfig {
    /// Per-step execution timeout. Specified in TOML as `timeout_per_step_ms` (milliseconds).
    #[serde(rename = "timeout_per_step_ms", deserialize_with = "duration_ms::deserialize")]
    pub timeout_per_step: Option<Duration>,

    /// Emit lifecycle events when `true` (Phase 2 — not yet wired to EventBus).
    pub emit_lifecycle_events: bool,

    /// Halt on the first step error when `true`; continue past errors when `false`.
    pub abort_on_error: bool,
}

mod duration_ms {
    use std::time::Duration;

    pub(super) fn deserialize<'de, D>(d: D) -> Result<Option<Duration>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        let ms = u64::deserialize(d)?;
        Ok(Some(Duration::from_millis(ms)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_config_default_happy() {
        let config = PipelineConfig::default();
        assert!(config.timeout_per_step.is_none());
        assert!(!config.emit_lifecycle_events);
        assert!(config.abort_on_error);
    }

    #[test]
    fn test_pipeline_config_custom_happy() {
        let config = PipelineConfig {
            timeout_per_step: Some(Duration::from_secs(5)),
            emit_lifecycle_events: true,
            abort_on_error: false,
        };
        assert_eq!(config.timeout_per_step, Some(Duration::from_secs(5)));
        assert!(config.emit_lifecycle_events);
        assert!(!config.abort_on_error);
    }
}
