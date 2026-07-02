use crate::api::{ParallelConfig, ParallelStepBuilder};

impl<Ctx: Send + 'static, E: Send + 'static> Default for ParallelStepBuilder<Ctx, E> {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            config: ParallelConfig::default(),
            event_bus: None,
        }
    }
}
