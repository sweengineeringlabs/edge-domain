/// Response for [`CompleteBootstrap::bootstrap_name`](crate::api::complete::traits::CompleteBootstrap::bootstrap_name).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteBootstrapNameResponse {
    /// Identifies this bootstrap implementation.
    pub name: String,
}
