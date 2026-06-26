//! [`DefaultStepRegistry<Ctx, E>`] — [`InMemoryRegistry`]-backed implementation of [`StepRegistry`].

use std::fmt;
use std::sync::Arc;

use edge_domain_registry::{InMemoryRegistry, Registry};

use crate::api::{Pipeline, PipelineDefinition, PipelineError, Step, StepRegistry};
use crate::core::pipeline::DefaultPipeline;

/// Step registry backed by [`InMemoryRegistry`] from `domain-registry`.
/// Resolves step names to shared instances at pipeline assembly time.
pub(crate) struct DefaultStepRegistry<Ctx, E> {
    inner: InMemoryRegistry<dyn Step<Ctx, E>>,
}

impl<Ctx, E> DefaultStepRegistry<Ctx, E>
where
    Ctx: Send + 'static,
    E: fmt::Display + fmt::Debug + Send + 'static,
{
    pub(crate) fn new() -> Self {
        Self {
            inner: InMemoryRegistry::new(),
        }
    }
}

impl<Ctx, E> StepRegistry for DefaultStepRegistry<Ctx, E>
where
    Ctx: Send + 'static,
    E: fmt::Display + fmt::Debug + Send + 'static,
{
    type Ctx = Ctx;
    type E = E;

    fn register(&mut self, name: &str, step: Arc<dyn Step<Ctx, E>>) {
        self.inner.register(name, step);
    }

    fn build_pipeline(
        &self,
        definition: &PipelineDefinition,
    ) -> Result<Box<dyn Pipeline<Ctx, E>>, PipelineError<E>> {
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
        let reg: DefaultStepRegistry<i32, String> = DefaultStepRegistry::new();
        assert!(reg.inner.is_empty());
    }

    /// @covers: register
    #[test]
    fn test_register_happy_adds_step() {
        let mut reg: DefaultStepRegistry<i32, String> = DefaultStepRegistry::new();
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
        let mut reg: DefaultStepRegistry<i32, String> = DefaultStepRegistry::new();
        reg.register("default", Arc::new(DefaultStep));
        reg.register("default", Arc::new(DefaultStep));
        assert_eq!(reg.inner.len(), 1);
    }
}
