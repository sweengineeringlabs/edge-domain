//! Validator service — opaque construction surface for [`Validator`](crate::api::Validator).

use std::sync::Arc;

use crate::api::Validator;
use crate::core::traits::DefaultValidator;

/// Identifies the validator `Service` implementation at runtime.
pub const VALIDATOR_SVC: &str = "validator";

/// Identifies the `ValidatorSvc` factory implementation.
pub const VALIDATOR_SVC_FACTORY: &str = "validator_svc_factory";

/// Construction handle for [`Validator`](crate::api::Validator) instances.
///
/// Consumers declare a dependency on `Box<dyn Validator>` (exclusive ownership)
/// or `Arc<dyn Validator>` (shared ownership) and obtain one through the
/// corresponding method. The concrete implementation (`DefaultValidator`) is never exposed.
///
/// # Examples
///
/// ## Exclusive ownership
///
/// ```rust,ignore
/// use edge_domain_pipeline::ValidatorSvc;
///
/// let validator = ValidatorSvc::create(true);
/// validator.validate(&config).await?;
/// ```
///
/// ## Shared ownership
///
/// ```rust,ignore
/// use edge_domain_pipeline::ValidatorSvc;
///
/// let validator = ValidatorSvc::create_shared(true);
/// let validator_clone = Arc::clone(&validator);
/// ```
pub struct ValidatorSvc;

impl ValidatorSvc {
    /// Create a validator with exclusive ownership.
    ///
    /// Pass `enabled: true` to enforce configuration validation before pipeline
    /// execution, or `false` to skip validation entirely.
    pub fn create(enabled: bool) -> Box<dyn Validator> {
        Box::new(DefaultValidator::new(enabled))
    }

    /// Create a validator with shared ownership.
    ///
    /// Use when the same validator instance must be shared across threads or services.
    pub fn create_shared(enabled: bool) -> Arc<dyn Validator> {
        Arc::new(DefaultValidator::new(enabled))
    }
}
