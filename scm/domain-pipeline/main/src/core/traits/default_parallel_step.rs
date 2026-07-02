//! [`DefaultParallelStep<Ctx, E>`] — runs a set of branch steps concurrently.

use std::sync::Arc;

use edge_domain_event::{DomainEvent, EventBus};
use tokio::task::JoinSet;

use crate::api::{
    ContextMutationRequest, ParallelBranchFailure, ParallelConfig, ParallelExecutor,
    ParallelStepError, Step, StepCountRequest, StepCountResponse, StepNameRequest,
};

const PARALLEL_STEP_STARTED: &str = "pipeline.step_started";
const PARALLEL_STEP_COMPLETED: &str = "pipeline.step_completed";
const PARALLEL_STEP_FAILED: &str = "pipeline.step_failed";

/// Outcome of one branch's execution, before it is turned into a
/// [`ParallelBranchFailure`] (or discarded, on success).
enum DefaultParallelStepBranchOutcome<E> {
    Done(Result<(), E>),
    TimedOut,
}

/// Runs a fixed set of branch steps concurrently against independent clones of the context.
///
/// See RFC-002 (`docs/3-architecture/rfc/RFC-002-parallel-step-execution.md`) for the full
/// design rationale. In short: `Ctx: Clone` because Rust's borrow checker correctly refuses
/// to hand out `&mut Ctx` to multiple concurrent branches — the engine performs no merge of
/// the mutated clones back into the caller's context; branch output that must be visible
/// afterward belongs behind `Arc<Mutex<T>>`/`Arc<RwLock<T>>` fields inside `Ctx` itself.
pub(crate) struct DefaultParallelStep<Ctx, E> {
    steps: Vec<Arc<dyn Step<Ctx = Ctx, ExecutionError = E>>>,
    config: ParallelConfig,
    event_bus: Option<Arc<dyn EventBus>>,
}

impl<Ctx: Send + 'static, E: Send + 'static> DefaultParallelStep<Ctx, E> {
    pub(crate) fn with_config(
        steps: Vec<Arc<dyn Step<Ctx = Ctx, ExecutionError = E>>>,
        config: ParallelConfig,
    ) -> Self {
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

#[async_trait::async_trait]
impl<Ctx, E> Step for DefaultParallelStep<Ctx, E>
where
    Ctx: Clone + Send + 'static,
    E: Send + 'static,
{
    type Ctx = Ctx;
    type ExecutionError = ParallelStepError<E>;

    async fn execute(
        &self,
        req: ContextMutationRequest<'_, Ctx>,
    ) -> Result<(), ParallelStepError<E>> {
        let mut join_set: JoinSet<(String, DefaultParallelStepBranchOutcome<E>)> = JoinSet::new();

        for step in &self.steps {
            let step_name = step
                .name(StepNameRequest)
                .map(|r| r.name)
                .unwrap_or_else(|_| "<unnamed>".to_string());
            let branch_step = Arc::clone(step);
            let mut ctx_clone = req.ctx.clone();
            let timeout = self.config.timeout_per_branch;
            let name_for_task = step_name.clone();

            self.emit(Arc::new(DefaultParallelStepBranchStartedEvt {
                step_name: step_name.clone(),
            }))
            .await;

            join_set.spawn(async move {
                let outcome = match timeout {
                    Some(dur) => match tokio::time::timeout(
                        dur,
                        branch_step.execute(ContextMutationRequest {
                            ctx: &mut ctx_clone,
                        }),
                    )
                    .await
                    {
                        Ok(r) => DefaultParallelStepBranchOutcome::Done(r),
                        Err(_elapsed) => DefaultParallelStepBranchOutcome::TimedOut,
                    },
                    None => DefaultParallelStepBranchOutcome::Done(
                        branch_step
                            .execute(ContextMutationRequest {
                                ctx: &mut ctx_clone,
                            })
                            .await,
                    ),
                };
                (name_for_task, outcome)
            });
        }

        let mut failures = Vec::new();

        while let Some(joined) = join_set.join_next().await {
            match joined {
                Ok((step_name, DefaultParallelStepBranchOutcome::Done(Ok(())))) => {
                    self.emit(Arc::new(DefaultParallelStepBranchCompletedEvt {
                        step_name,
                    }))
                    .await;
                }
                Ok((step_name, DefaultParallelStepBranchOutcome::Done(Err(cause)))) => {
                    self.emit(Arc::new(DefaultParallelStepBranchFailedEvt {
                        step_name: step_name.clone(),
                    }))
                    .await;
                    failures.push(ParallelBranchFailure::Failed(crate::api::StepError {
                        step_name,
                        cause,
                    }));
                    if self.config.fail_fast {
                        join_set.abort_all();
                        break;
                    }
                }
                Ok((step_name, DefaultParallelStepBranchOutcome::TimedOut)) => {
                    self.emit(Arc::new(DefaultParallelStepBranchFailedEvt {
                        step_name: step_name.clone(),
                    }))
                    .await;
                    failures.push(ParallelBranchFailure::TimedOut { step_name });
                    if self.config.fail_fast {
                        join_set.abort_all();
                        break;
                    }
                }
                Err(join_error) => {
                    if join_error.is_cancelled() {
                        // Our own abort_all() from a prior fail_fast failure — not a new one.
                        continue;
                    }
                    failures.push(ParallelBranchFailure::Panicked);
                    if self.config.fail_fast {
                        join_set.abort_all();
                        break;
                    }
                }
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(ParallelStepError { failures })
        }
    }
}

impl<Ctx, E> ParallelExecutor for DefaultParallelStep<Ctx, E>
where
    Ctx: Clone + Send + 'static,
    E: Send + 'static,
{
    type BranchError = E;

    fn branch_count(
        &self,
        _req: StepCountRequest,
    ) -> Result<StepCountResponse, crate::api::PipelineError<E>> {
        Ok(StepCountResponse {
            count: self.steps.len(),
        })
    }
}

struct DefaultParallelStepBranchStartedEvt {
    step_name: String,
}

impl DomainEvent for DefaultParallelStepBranchStartedEvt {
    fn event_type(&self) -> &str {
        PARALLEL_STEP_STARTED
    }

    fn aggregate_id(&self) -> &str {
        self.step_name.as_str()
    }
}

struct DefaultParallelStepBranchCompletedEvt {
    step_name: String,
}

impl DomainEvent for DefaultParallelStepBranchCompletedEvt {
    fn event_type(&self) -> &str {
        PARALLEL_STEP_COMPLETED
    }

    fn aggregate_id(&self) -> &str {
        self.step_name.as_str()
    }
}

struct DefaultParallelStepBranchFailedEvt {
    step_name: String,
}

impl DomainEvent for DefaultParallelStepBranchFailedEvt {
    fn event_type(&self) -> &str {
        PARALLEL_STEP_FAILED
    }

    fn aggregate_id(&self) -> &str {
        self.step_name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_event::InProcessEventBus;

    /// @covers: with_config
    #[test]
    fn test_with_config_happy_stores_steps_and_config() {
        let config = ParallelConfig {
            fail_fast: true,
            ..ParallelConfig::default()
        };
        let step: DefaultParallelStep<i32, String> =
            DefaultParallelStep::with_config(Vec::new(), config);
        assert_eq!(step.steps.len(), 0);
        assert!(step.config.fail_fast);
    }

    /// @covers: with_config
    #[test]
    fn test_with_config_edge_no_event_bus_by_default() {
        let step: DefaultParallelStep<i32, String> =
            DefaultParallelStep::with_config(Vec::new(), ParallelConfig::default());
        assert!(step.event_bus.is_none());
    }

    /// @covers: with_event_bus
    #[test]
    fn test_with_event_bus_happy_stores_cloned_arc() {
        let bus: Arc<dyn EventBus> = Arc::new(InProcessEventBus::new(4));
        let initial_count = Arc::strong_count(&bus);
        let step: DefaultParallelStep<i32, String> =
            DefaultParallelStep::with_config(Vec::new(), ParallelConfig::default())
                .with_event_bus(Arc::clone(&bus));
        assert_eq!(
            Arc::strong_count(&bus),
            initial_count + 1,
            "with_event_bus must retain the cloned Arc"
        );
        assert!(step.event_bus.is_some());
    }

    /// @covers: emit
    #[tokio::test]
    async fn test_emit_happy_publishes_when_enabled_with_bus() {
        let bus = Arc::new(InProcessEventBus::new(4));
        let mut receiver = bus.subscribe();
        let config = ParallelConfig {
            emit_lifecycle_events: true,
            ..ParallelConfig::default()
        };
        let step: DefaultParallelStep<i32, String> =
            DefaultParallelStep::with_config(Vec::new(), config)
                .with_event_bus(bus as Arc<dyn EventBus>);

        step.emit(Arc::new(DefaultParallelStepBranchStartedEvt {
            step_name: "probe".to_string(),
        }))
        .await;

        let event = tokio::time::timeout(std::time::Duration::from_secs(1), receiver.recv())
            .await
            .expect("must receive before timeout")
            .expect("recv must not error");
        assert_eq!(event.event_type(), PARALLEL_STEP_STARTED);
    }

    /// @covers: emit
    #[tokio::test]
    async fn test_emit_edge_noop_when_disabled() {
        let bus = Arc::new(InProcessEventBus::new(4));
        let mut receiver = bus.subscribe();
        let step: DefaultParallelStep<i32, String> =
            DefaultParallelStep::with_config(Vec::new(), ParallelConfig::default())
                .with_event_bus(bus as Arc<dyn EventBus>);

        step.emit(Arc::new(DefaultParallelStepBranchStartedEvt {
            step_name: "probe".to_string(),
        }))
        .await;

        let outcome =
            tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv()).await;
        assert!(
            outcome.is_err(),
            "emit must no-op when emit_lifecycle_events is false"
        );
    }
}
