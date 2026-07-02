//! [`RenderRequest`] — request for [`Prompt::render`](crate::api::prompt::traits::Prompt::render).

use crate::api::prompt::types::RenderContext;

/// Request to render a template against `context`.
#[derive(Debug, PartialEq)]
pub struct RenderRequest<'a> {
    /// The context to render against.
    pub context: &'a RenderContext,
}
