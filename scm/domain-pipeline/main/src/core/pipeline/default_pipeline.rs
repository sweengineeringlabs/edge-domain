//! [`DefaultPipeline<Ctx, E>`] — orchestrates sequential step execution.

use std::fmt;
use std::sync::Arc;

use edge_domain_event::{DomainEvent, EventBus};
use edge_domain_service::{Service, ServiceError};
use futures::future::BoxFuture;
use tokio::time;

use crate::api::{Pipeline, PipelineConfig, PipelineError, Step, StepError};

// ── DefaultPipeline ───────────────────────────────────────────────────────────

/// Executes a sequence of steps in order, passing context through each.
#[derive(Clone)]
pub(crate) struct DefaultPipeline<Ctx, E> {
    steps: Vec<Arc<dyn Step<Ctx, E>>>,
    config: PipelineConfig,
    event_bus: Option<Arc<dyn EventBus>>,
}

impl<Ctx: Send + 'static, E: fmt::Display + fmt::Debug + Send + 'static> DefaultPipeline<Ctx, E> {
    pub(crate) fn with_config(steps: Vec<Arc<dyn Step<Ctx, E>>>, config: PipelineConfig) -> Self {
        Self {
            steps,
            config,
            event_bus: None,
        }
    }

    pub(crate) fn with_event_bus(mut self, bus: Arc<dyn EventBus>) -> Self {
        self.event_bus = Some(bus);
        self
    }

    async fn emit(&self, event: Arc<dyn DomainEvent>) {
        if self.config.emit_lifecycle_events {
            if let Some(bus) = &self.event_bus {
                let _ = bus.publish(event).await;
            }
        }
    }
}

/// `Service` impl — exposes `DefaultPipeline` to the dispatcher bridge.
///
/// `Service::execute` takes ownership of `Ctx`, delegates to `Pipeline::run(&mut ctx)`,
/// then returns the mutated context. `PipelineError<E>` maps to `ServiceError::Internal`.
impl<Ctx: Send + 'static, E: fmt::Display + fmt::Debug + Send + 'static> Service
    for DefaultPipeline<Ctx, E>
{
    type Request = Ctx;
    type Response = Ctx;

    fn name(&self) -> &str {
        crate::PIPELINE_SVC
    }

    fn execute(&self, ctx: Ctx) -> BoxFuture<'_, Result<Ctx, ServiceError>> {
        Box::pin(async move {
            let mut ctx = ctx;
            Pipeline::run(self, &mut ctx)
                .await
                .map(|_| ctx)
                .map_err(|e| ServiceError::Internal(e.to_string()))
        })
    }
}

#[async_trait::async_trait]
impl<Ctx: Send + 'static, E: fmt::Display + fmt::Debug + Send + 'static> Pipeline<Ctx, E>
    for DefaultPipeline<Ctx, E>
{
    async fn run(&self, ctx: &mut Ctx) -> Result<(), PipelineError<E>> {
        for step in &self.steps {
            let step_name = step.name().to_string();

            self.emit(Arc::new(DefaultPipelineStepStartedEvt {
                step_name: step_name.clone(),
            }))
            .await;

            let step_result: Result<(), E> = match self.config.timeout_per_step {
                Some(dur) => match time::timeout(dur, step.execute(ctx)).await {
                    Ok(r) => r,
                    Err(_elapsed) => {
                        self.emit(Arc::new(DefaultPipelineStepFailedEvt {
                            step_name: step_name.clone(),
                        }))
                        .await;
                        if self.config.abort_on_error {
                            return Err(PipelineError::StepTimeout { step_name });
                        }
                        continue;
                    }
                },
                None => step.execute(ctx).await,
            };

            match step_result {
                Ok(()) => {
                    self.emit(Arc::new(DefaultPipelineStepCompletedEvt {
                        step_name,
                    }))
                    .await;
                }
                Err(cause) => {
                    self.emit(Arc::new(DefaultPipelineStepFailedEvt {
                        step_name: step_name.clone(),
                    }))
                    .await;
                    if self.config.abort_on_error {
                        return Err(PipelineError::StepFailed(StepError { step_name, cause }));
                    }
                }
            }
        }
        Ok(())
    }

    fn step_count(&self) -> usize {
        self.steps.len()
    }

    fn config(&self) -> &PipelineConfig {
        &self.config
    }
}

// ── private lifecycle event types ─────────────────────────────────────────────

const PIPELINE_STEP_STARTED: &str = "pipeline.step_started";
const PIPELINE_STEP_COMPLETED: &str = "pipeline.step_completed";
const PIPELINE_STEP_FAILED: &str = "pipeline.step_failed";

struct DefaultPipelineStepStartedEvt {
    step_name: String,
}

impl DomainEvent for DefaultPipelineStepStartedEvt {
    fn event_type(&self) -> &str {
        PIPELINE_STEP_STARTED
    }

    fn aggregate_id(&self) -> &str {
        self.step_name.as_str()
    }
}

struct DefaultPipelineStepCompletedEvt {
    step_name: String,
}

impl DomainEvent for DefaultPipelineStepCompletedEvt {
    fn event_type(&self) -> &str {
        PIPELINE_STEP_COMPLETED
    }

    fn aggregate_id(&self) -> &str {
        self.step_name.as_str()
    }
}

struct DefaultPipelineStepFailedEvt {
    step_name: String,
}

impl DomainEvent for DefaultPipelineStepFailedEvt {
    fn event_type(&self) -> &str {
        PIPELINE_STEP_FAILED
    }

    fn aggregate_id(&self) -> &str {
        self.step_name.as_str()
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_event::InProcessEventBus;

    /// @covers: with_config
    #[test]
    fn test_new_happy_creates_empty() {
        let pipeline: DefaultPipeline<i32, String> =
            DefaultPipeline::with_config(vec![], PipelineConfig::default());
        assert_eq!(pipeline.step_count(), 0);
    }

    /// @covers: with_config
    #[test]
    fn test_with_config_happy_sets_timeout() {
        let config = PipelineConfig {
            timeout_per_step: Some(std::time::Duration::from_secs(5)),
            emit_lifecycle_events: true,
            abort_on_error: false,
        };
        let pipeline: DefaultPipeline<i32, String> =
            DefaultPipeline::with_config(vec![], config.clone());
        assert_eq!(
            pipeline.config().timeout_per_step,
            Some(std::time::Duration::from_secs(5))
        );
    }

    /// @covers: config
    #[test]
    fn test_config_happy_returns_defaults() {
        let pipeline: DefaultPipeline<i32, String> =
            DefaultPipeline::with_config(vec![], PipelineConfig::default());
        let config = pipeline.config();
        assert!(config.timeout_per_step.is_none());
        assert!(!config.emit_lifecycle_events);
        assert!(config.abort_on_error);
    }

    /// @covers: name
    #[test]
    fn test_service_name_happy_returns_pipeline_svc() {
        let pipeline: DefaultPipeline<i32, String> =
            DefaultPipeline::with_config(vec![], PipelineConfig::default());
        assert_eq!(Service::name(&pipeline), crate::PIPELINE_SVC);
    }

    /// @covers: with_event_bus
    #[test]
    fn test_with_event_bus_happy_stores_cloned_arc() {
        let bus: Arc<dyn EventBus> = Arc::new(InProcessEventBus::new(4));
        let initial_count = Arc::strong_count(&bus);
        let pipeline: DefaultPipeline<i32, String> =
            DefaultPipeline::with_config(vec![], PipelineConfig::default())
                .with_event_bus(Arc::clone(&bus));
        assert_eq!(
            Arc::strong_count(&bus),
            initial_count + 1,
            "with_event_bus must retain the cloned Arc"
        );
        assert!(pipeline.event_bus.is_some());
    }

    /// @covers: with_event_bus
    #[test]
    fn test_with_event_bus_error_second_call_replaces_first() {
        let bus1: Arc<dyn EventBus> = Arc::new(InProcessEventBus::new(4));
        let bus2: Arc<dyn EventBus> = Arc::new(InProcessEventBus::new(8));
        let count1_before = Arc::strong_count(&bus1);
        let count2_before = Arc::strong_count(&bus2);
        let pipeline: DefaultPipeline<i32, String> =
            DefaultPipeline::with_config(vec![], PipelineConfig::default())
                .with_event_bus(Arc::clone(&bus1))
                .with_event_bus(Arc::clone(&bus2));
        assert_eq!(Arc::strong_count(&bus1), count1_before, "first bus must be released");
        assert_eq!(Arc::strong_count(&bus2), count2_before + 1, "second bus must be retained");
        assert!(pipeline.event_bus.is_some());
    }

    /// @covers: with_event_bus
    #[test]
    fn test_with_event_bus_edge_absent_without_call() {
        let pipeline: DefaultPipeline<i32, String> =
            DefaultPipeline::with_config(vec![], PipelineConfig::default());
        assert!(
            pipeline.event_bus.is_none(),
            "event_bus must be None when with_event_bus was never called"
        );
    }
}
