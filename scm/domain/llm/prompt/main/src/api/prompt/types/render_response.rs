//! [`RenderResponse`] — response for [`Prompt::render`](crate::api::prompt::traits::Prompt::render).

/// The rendered template output.
#[derive(Debug, PartialEq)]
pub struct RenderResponse {
    /// The rendered text.
    pub rendered: String,
}
