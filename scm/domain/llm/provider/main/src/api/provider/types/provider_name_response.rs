/// Response for [`Provider::name`](crate::api::provider::traits::Provider::name).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderNameResponse {
    /// Stable identifier for the provider (e.g. `"anthropic"`).
    pub name: String,
}
