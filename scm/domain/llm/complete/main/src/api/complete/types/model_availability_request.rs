/// Request for [`Completer::is_model_available`](crate::api::complete::traits::Completer::is_model_available).
#[derive(Debug, Clone, Copy)]
pub struct ModelAvailabilityRequest<'a> {
    /// Model id being queried.
    pub model: &'a str,
}
