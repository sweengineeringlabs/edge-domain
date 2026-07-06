//! [`PromptBootstrapNameResponse`] — response for [`PromptBootstrap::bootstrap_name`](crate::api::prompt::traits::PromptBootstrap::bootstrap_name).

/// This bootstrap implementation's identifier.
#[derive(Debug, PartialEq)]
pub struct PromptBootstrapNameResponse {
    /// Stable identifier for this bootstrap implementation.
    pub name: &'static str,
}
