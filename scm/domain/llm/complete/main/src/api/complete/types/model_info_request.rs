/// Request for [`Completer::model_info`](crate::api::complete::traits::Completer::model_info)
/// and [`ModelOps::find_model`](crate::api::complete::traits::ModelOps::find_model).
#[derive(Debug, Clone, Copy)]
pub struct ModelInfoRequest<'a> {
    /// Model id to look up.
    pub model: &'a str,
}
