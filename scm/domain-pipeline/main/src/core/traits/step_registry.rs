//! [`DefaultStepRegistry`] — [`InMemoryRegistry`]-backed implementation of [`StepRegistry`].

use std::sync::Arc;

use edge_domain_registry::{InMemoryRegistry, Registry};

use crate::api::{Pipeline, PipelineDefinition, PipelineError, Step, StepRegistry};
use crate::core::pipeline::DefaultPipeline;

/// Step registry backed by [`InMemoryRegistry`] from `domain-registry`.
/// Resolves step names to shared instances at pipeline assembly time.
pub(crate) struct DefaultStepRegistry<Ctx> {
    inner: InMemoryRegistry<dyn Step<Ctx>>,
}

impl<Ctx: Send + 'static> DefaultStepRegistry<Ctx> {
    pub(crate) fn new() -> Self {
        Self {
            inner: InMemoryRegistry::new(),
        }
    }
}

// impl StepRegistry for DefaultStepRegistry — generic over Ctx; see full signature below.
impl<Ctx: Send + 'static> StepRegistry<Ctx> for DefaultStepRegistry<Ctx> {
    fn register(&mut self, name: &str, step: Arc<dyn Step<Ctx>>) {
        self.inner.register(name, step);
    }

    fn build_pipeline(
        &self,
        definition: &PipelineDefinition,
    ) -> Result<Box<dyn Pipeline<Ctx>>, PipelineError> {
        let mut steps = Vec::with_capacity(definition.steps.len());
        for name in &definition.steps {
            match self.inner.get(name.as_str()) {
                Some(step) => steps.push(step),
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
        assert!(reg.inner.is_empty());
    }

    /// @covers: register
    #[test]
    fn test_register_happy_adds_step() {
        let mut reg: DefaultStepRegistry<i32> = DefaultStepRegistry::new();
        reg.register("default", Arc::new(DefaultStep));
        let found = reg.inner.get("default");
        assert!(found.is_some(), "step must be stored under the registered name");
        assert_eq!(
            found.map(|s| s.name().to_owned()),
            Some("default-step".to_owned()),
            "retrieved step must report the expected name"
        );
    }

    /// @covers: register
    #[test]
    fn test_register_error_duplicate_overwrites() {
        let mut reg: DefaultStepRegistry<i32> = DefaultStepRegistry::new();
        reg.register("default", Arc::new(DefaultStep));
        reg.register("default", Arc::new(DefaultStep));
        assert_eq!(reg.inner.len(), 1);
    }
}
