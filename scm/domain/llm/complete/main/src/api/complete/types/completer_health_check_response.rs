/// Response for [`Completer::health_check`](crate::api::complete::traits::Completer::health_check).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompleterHealthCheckResponse {
    /// Whether the completer is healthy and can process requests.
    pub healthy: bool,
}
