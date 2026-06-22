//! Service Abstraction Framework — public API surface.

pub use crate::api::{Pipeline, Step, PipelineError};
pub use crate::core::{DefaultPipeline, PipelineConfig};
pub use crate::spi::{PipelineBuilder, NoopStep, AlwaysPassStep, AlwaysFailStep, MutatingStep};
