//! [`DefaultStepRegistry`] — HashMap-backed implementation of [`StepRegistry`].

use std::collections::HashMap;
use std::sync::Arc;

use crate::api::{Pipeline, PipelineDefinition, PipelineError, Step, StepRegistry};
use crate::core::pipeline::DefaultPipeline;

/// HashMap-backed step registry. Resolves step names to shared instances at pipeline
/// assembly time.
pub(crate) struct DefaultStepRegistry<Ctx> {
    steps: HashMap<String, Arc<dyn Step<Ctx>>>,
}

impl<Ctx> DefaultStepRegistry<Ctx> {
    pub(crate) fn new() -> Self {
        Self {
            steps: HashMap::new(),
        }
    }
}

// impl StepRegistry for DefaultStepRegistry — generic over Ctx; see full signature below.
impl<Ctx: Send + 'static> StepRegistry<Ctx> for DefaultStepRegistry<Ctx> {
    fn register(&mut self, name: &str, step: Arc<dyn Step<Ctx>>) {
        self.steps.insert(name.to_owned(), step);
    }

    fn build_pipeline(
        &self,
        definition: &PipelineDefinition,
    ) -> Result<Box<dyn Pipeline<Ctx>>, PipelineError> {
        let mut steps = Vec::with_capacity(definition.steps.len());
        for name in &definition.steps {
            match self.steps.get(name.as_str()) {
                Some(step) => steps.push(Arc::clone(step)),
                None => return Err(PipelineError::UnknownStep(name.clone())),
            }
        }
        Ok(Box::new(DefaultPipeline::with_config(steps, definition.config.clone())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_steps::default_step::DefaultStep;

    /// @covers: new
    #[test]
    fn test_new_happy_starts_empty() {
        let reg: DefaultStepRegistry<i32> = DefaultStepRegistry::new();
        assert!(reg.steps.is_empty());
    }

    /// @covers: register
    #[test]
    fn test_register_happy_adds_step() {
        let mut reg: DefaultStepRegistry<i32> = DefaultStepRegistry::new();
        reg.register("default", Arc::new(DefaultStep));
        assert!(reg.steps.contains_key("default"));
    }

    /// @covers: register
    #[test]
    fn test_register_error_duplicate_overwrites() {
        let mut reg: DefaultStepRegistry<i32> = DefaultStepRegistry::new();
        reg.register("default", Arc::new(DefaultStep));
        reg.register("default", Arc::new(DefaultStep));
        assert_eq!(reg.steps.len(), 1);
    }
}
