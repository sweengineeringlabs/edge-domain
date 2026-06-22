//! [`PipelineConfig`] — configuration for pipeline execution.

use std::time::Duration;

/// Configuration for pipeline execution behavior.
#[derive(Clone, Debug)]
pub struct PipelineConfig {
    /// Per-step timeout (optional).
    pub timeout_per_step: Option<Duration>,

    /// Emit lifecycle events (StepStarted, StepCompleted, etc).
    pub emit_lifecycle_events: bool,

    /// Abort on error (default: true). If false, silently skip failed steps.
    pub abort_on_error: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            timeout_per_step: None,
            emit_lifecycle_events: false,
            abort_on_error: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_config_default() {
        let config = PipelineConfig::default();
        assert!(config.timeout_per_step.is_none());
        assert!(!config.emit_lifecycle_events);
        assert!(config.abort_on_error);
    }

    #[test]
    fn test_pipeline_config_custom() {
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
