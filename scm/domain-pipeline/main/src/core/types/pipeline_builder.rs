use crate::api::{PipelineBuilder, PipelineConfig};

impl<Ctx: Send + 'static> Default for PipelineBuilder<Ctx> {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            config: PipelineConfig::default(),
            event_bus: None,
        }
    }
}
