//! Pipeline orchestration — core implementations.

pub(crate) mod concrete_pipeline;
pub(crate) mod default_pipeline;

pub(crate) use concrete_pipeline::ConcretePipeline;
pub(crate) use default_pipeline::DefaultPipeline;
