//! [`CacheBuildRequest`] — request for [`Prompt::cache`](crate::api::prompt::traits::Prompt::cache).

use crate::api::prompt::types::RenderContext;

/// Request to build a cache entry for an already-rendered prompt.
#[derive(Debug, PartialEq)]
pub struct CacheBuildRequest<'a> {
    /// The context the prompt was rendered against.
    pub context: &'a RenderContext,
    /// The already-rendered output.
    pub rendered: String,
}
