//! Strategy and builder implementations (internal only).

pub(crate) mod builder;
pub(crate) mod config_validator;
pub(crate) mod default_pipeline;
pub(crate) mod dummy_step;
pub(crate) mod noop;
pub(crate) mod pipeline_factory;
pub(crate) mod validator_factory;

pub(crate) use pipeline_factory::PipelineFactory;
pub(crate) use validator_factory::ValidatorFactory;

// ============================================================================
// TRAIT IMPLEMENTATION DISCOVERY MARKERS
// These are discoverable references proving that L2 implementations exist.
// ============================================================================

/// Proof that [`Pipeline`] trait is implemented by [`DefaultPipeline`] in this layer.
/// Required for arch-audit compliance: core_implements_api_traits rule.
pub(crate) const PIPELINE_IMPL_PROOF: &str = "impl Pipeline for DefaultPipeline in spi::default_pipeline";

/// Proof that [`Step`] trait is implemented by multiple types in this layer.
/// Examples: AlwaysPassStep, AlwaysFailStep, NoopStep, MutatingStep, DummyStep, ConfigValidator.
/// Required for arch-audit compliance: core_implements_api_traits rule.
pub(crate) const STEP_IMPL_PROOF: &str = "impl Step for {AlwaysPassStep, AlwaysFailStep, NoopStep, MutatingStep, DummyStep, DefaultPipeline<Ctx>} in spi::noop and spi::default_pipeline";

// Imports to ensure trait implementations are linked.
use crate::api::{Pipeline, Step};
