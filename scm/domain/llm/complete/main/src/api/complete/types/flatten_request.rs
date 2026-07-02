use crate::api::complete::types::MessageContent;

/// Request for [`ContentFlattener::flatten`](crate::api::complete::traits::ContentFlattener::flatten).
#[derive(Debug, Clone, Copy)]
pub struct FlattenRequest<'a> {
    /// Message content to flatten.
    pub content: &'a MessageContent,
}
