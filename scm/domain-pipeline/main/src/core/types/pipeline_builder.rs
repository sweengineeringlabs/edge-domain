use crate::api::{PipelineBuilder, PipelineConfig};

impl<Ctx: Send + 'static, E: Send + 'static> Default for PipelineBuilder<Ctx, E> {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            config: PipelineConfig::default(),
            event_bus: None,
        }
    }
}
