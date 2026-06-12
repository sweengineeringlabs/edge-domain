//! [`HandlerFactory`] — config-driven handler constructor contract.

use crate::api::handler::HandlerError;

/// Constructs a [`Handler`](crate::Handler) from a typed configuration value.
///
/// Implement this trait on your handler type to enable config-driven assembly via
/// `FeatureRegistryExt::build_handler` in the assembler.
///
/// # Example
///
/// ```rust,no_run
/// use edge_domain::{Handler, HandlerError, HandlerFactory};
/// use async_trait::async_trait;
///
/// struct GuardConfig { token: String }
/// struct GuardHandler { token: String }
///
/// impl HandlerFactory<GuardConfig> for GuardHandler {
///     fn build(cfg: GuardConfig) -> Result<Self, HandlerError> {
///         Ok(GuardHandler { token: cfg.token })
///     }
/// }
/// ```
pub trait HandlerFactory<Config>: Sized {
    /// Build a handler instance from a validated configuration value.
    ///
    /// Called by `FeatureRegistry::build_handler` when the feature is enabled.
    /// Validation of the config has already been applied by `OptionalSection::validate_enabled`.
    fn build(cfg: Config) -> Result<Self, HandlerError>;
}
