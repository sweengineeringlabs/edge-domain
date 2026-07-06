use crate::api::types::MessageContent;

/// Response for [`Skill::render_content`](crate::api::traits::Skill::render_content).
pub struct RenderContentResponse {
    /// The rendered message body.
    pub content: Box<MessageContent>,
}
