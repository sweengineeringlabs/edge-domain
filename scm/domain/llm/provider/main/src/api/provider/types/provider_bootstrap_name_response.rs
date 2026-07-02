/// Response for [`ProviderBootstrap::bootstrap_name`](crate::api::provider::traits::ProviderBootstrap::bootstrap_name).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderBootstrapNameResponse {
    /// Identifies this bootstrap implementation.
    pub name: String,
}
