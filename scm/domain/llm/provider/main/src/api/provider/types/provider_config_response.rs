use crate::api::provider::types::ProviderConfig;

/// Response for [`Provider::provider_config`](crate::api::provider::traits::Provider::provider_config).
#[derive(Debug, Clone)]
pub struct ProviderConfigResponse {
    /// Configuration the provider was constructed with.
    pub config: Box<ProviderConfig>,
}
