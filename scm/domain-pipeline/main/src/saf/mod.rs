//! Service Abstraction Framework — public API surface.

mod pipeline_svc;

pub use pipeline_svc::{DefaultPipeline, Pipeline, PipelineConfig, PipelineError, Step};
pub use crate::spi::{PipelineBuilder, NoopStep, AlwaysPassStep, AlwaysFailStep, MutatingStep};
