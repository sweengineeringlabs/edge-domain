//! [`PipelineAssemblyResponse`] — wraps a successfully assembled pipeline.

use crate::api::Pipeline;

/// Response carrying a boxed, ready-to-run pipeline.
pub struct PipelineAssemblyResponse<Ctx, E> {
    /// The assembled pipeline.
    pub pipeline: Box<dyn Pipeline<Ctx = Ctx, E = E, Request = Ctx, Response = Ctx>>,
}
