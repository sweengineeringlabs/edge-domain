/// Response for [`Completer::supports`](crate::api::complete::traits::Completer::supports).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelSupportResponse {
    /// Whether the queried model id is supported.
    pub supported: bool,
}
