/// Response for [`Completer::is_model_available`](crate::api::complete::traits::Completer::is_model_available).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelAvailabilityResponse {
    /// Whether the queried model can be reached right now.
    pub available: bool,
}
