/// Response for [`ContentFlattener::flatten`](crate::api::complete::traits::ContentFlattener::flatten).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlattenResponse {
    /// Text representation of the flattened content.
    pub text: String,
}
