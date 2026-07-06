use crate::api::types::ContentPart;

/// Request for [`Skill::render_content`](crate::api::traits::Skill::render_content).
#[derive(Debug, Clone)]
pub struct RenderContentRequest {
    /// The content parts to render.
    pub parts: Vec<ContentPart>,
}
