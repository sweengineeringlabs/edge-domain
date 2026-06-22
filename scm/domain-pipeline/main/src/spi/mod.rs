//! Strategy and builder implementations.

mod pipeline_builder;
mod noop_step;

pub use pipeline_builder::PipelineBuilder;
pub use noop_step::{NoopStep, AlwaysPassStep, AlwaysFailStep, MutatingStep};
