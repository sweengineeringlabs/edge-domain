/// Request for [`Completer::supports`](crate::api::complete::traits::Completer::supports).
#[derive(Debug, Clone, Copy)]
pub struct ModelSupportRequest<'a> {
    /// Model id being queried.
    pub model: &'a str,
}
