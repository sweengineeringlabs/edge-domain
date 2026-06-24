//! Service Abstraction Framework — one `<trait>_svc.rs` per api port.
//!
//! Each file re-exports the trait from api/ and exposes its service-identity constant.

pub mod pipeline_svc;
pub mod step_svc;
pub mod validator_svc;

pub use pipeline_svc::{Pipeline, PIPELINE_SVC};
pub use step_svc::{Step, STEP_SVC};
pub use validator_svc::{Validator, VALIDATOR_SVC};
