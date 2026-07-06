/// Response for [`ReasoningBootstrap::bootstrap_name`](crate::api::reasoning::traits::ReasoningBootstrap::bootstrap_name).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningBootstrapNameResponse {
    /// Identifies this bootstrap implementation.
    pub name: String,
}
