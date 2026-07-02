use std::time::Duration;

use swe_edge_configbuilder::ConfigSection;

use crate::api::PipelineConfig;

const PIPELINE_SECTION_NAME: &str = "pipeline";

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            timeout_per_step: None,
            emit_lifecycle_events: false,
            abort_on_error: true,
        }
    }
}

impl ConfigSection for PipelineConfig {
    fn section_name() -> &'static str {
        PIPELINE_SECTION_NAME
    }
}

/// TOML wire shape for [`PipelineConfig`] — mirrors the public struct but keeps
/// `timeout_per_step_ms` as raw milliseconds so `serde` doesn't need a custom
/// `Deserializer`-bound helper inside `api/`.
#[derive(serde::Deserialize)]
#[serde(default)]
struct RawPipelineConfig {
    timeout_per_step_ms: Option<u64>,
    emit_lifecycle_events: bool,
    abort_on_error: bool,
}

impl Default for RawPipelineConfig {
    fn default() -> Self {
        let config = PipelineConfig::default();
        Self {
            timeout_per_step_ms: config.timeout_per_step.map(|d| d.as_millis() as u64),
            emit_lifecycle_events: config.emit_lifecycle_events,
            abort_on_error: config.abort_on_error,
        }
    }
}

impl<'de> serde::Deserialize<'de> for PipelineConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = RawPipelineConfig::deserialize(deserializer)?;
        Ok(Self {
            timeout_per_step: raw.timeout_per_step_ms.map(Duration::from_millis),
            emit_lifecycle_events: raw.emit_lifecycle_events,
            abort_on_error: raw.abort_on_error,
        })
    }
}
