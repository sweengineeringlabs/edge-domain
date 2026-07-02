//! Service Abstraction Framework — one `<trait>_svc.rs` per api port.
//!
//! Each file re-exports the trait from api/ and exposes its service-identity constant.

pub mod pipeline_svc_factory;
mod step;
pub mod validator_svc_factory;

pub use pipeline_svc_factory::{PipelineSvc, PIPELINE_SVC, PIPELINE_SVC_FACTORY};
pub use step::{
    StepRegistrySvc, StepSvc, STEP_REGISTRY_SVC, STEP_REGISTRY_SVC_FACTORY, STEP_SVC,
    STEP_SVC_FACTORY,
};
pub use validator_svc_factory::{ValidatorSvc, VALIDATOR_SVC, VALIDATOR_SVC_FACTORY};
