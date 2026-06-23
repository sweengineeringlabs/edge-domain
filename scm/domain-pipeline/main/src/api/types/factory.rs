use std::sync::Arc;
use crate::api::{Pipeline, Step, Validator};

pub struct PipelineFactory;

impl PipelineFactory {
    pub fn create<Ctx: Send + 'static>(steps: Vec<Arc<dyn Step<Ctx>>>) -> Box<dyn Pipeline<Ctx>> {
        crate::saf::PipelineService::create_pipeline(steps)
    }

    pub fn create_with_config<Ctx: Send + 'static'>(
        steps: Vec<Arc<dyn Step<Ctx>>>,
        config: crate::api::PipelineConfig,
    ) -> Box<dyn Pipeline<Ctx>> {
        crate::saf::PipelineService::create_pipeline_with_config(steps, config)
    }
}

pub struct ValidatorFactory;

impl ValidatorFactory {
    pub fn create(enabled: bool) -> Box<dyn Validator> {
        crate::saf::ValidatorService::create_validator(enabled)
    }
}
