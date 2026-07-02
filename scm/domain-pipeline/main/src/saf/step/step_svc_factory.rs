//! Step service — opaque construction surface for [`Step`](crate::api::Step).

use std::sync::Arc;

use crate::api::Step;
use crate::core::step::DefaultStep;

/// Identifies the step `Service` implementation at runtime.
pub const STEP_SVC: &str = "step";

/// Identifies the step SAF factory.
pub const STEP_SVC_FACTORY: &str = "step_svc_factory";

/// Construction handle for [`Step`](crate::api::Step) instances.
///
/// Consumers declare a dependency on `Box<dyn Step<Ctx, E>>` (exclusive ownership) or
/// `Arc<dyn Step<Ctx, E>>` (shared ownership). The concrete implementation
/// (`DefaultStep`) is never exposed.
pub struct StepSvc;

impl StepSvc {
    /// Build a no-op step (leaves the context unchanged, always succeeds) with exclusive
    /// ownership.
    ///
    /// Useful as a placeholder when a pipeline position needs a step but no work is
    /// required.
    pub fn noop<Ctx, E>() -> Box<dyn Step<Ctx, E>>
    where
        Ctx: Send + 'static,
        E: Send + 'static,
    {
        Box::new(DefaultStep)
    }

    /// Build a no-op step with shared ownership.
    pub fn noop_shared<Ctx, E>() -> Arc<dyn Step<Ctx, E>>
    where
        Ctx: Send + 'static,
        E: Send + 'static,
    {
        Arc::new(DefaultStep)
    }
}
