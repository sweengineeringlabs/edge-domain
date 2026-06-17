//! `PromptEndpoint` — connects the prompt primitive to the edge dispatch pipeline.
//!
//! Per ADR-037 this single type carries both faces of a connected native:
//! it implements `Handler` (register the domain + ride the dispatch pipeline)
//! and `Service` (typed, named consumption), with the `Service` face delegating
//! into the `Handler` (Service → Dispatch → Handler → core).

use crate::api::prompt::types::StaticPrompt;

/// Pipeline endpoint for the prompt primitive.
///
/// Wraps a [`StaticPrompt`] and exposes it as both a dispatchable `Handler`
/// and a typed `Service`. The connected operation is template rendering:
/// `RenderContext` in, the rendered `String` out.
#[derive(Clone, Debug)]
pub struct PromptEndpoint {
    pub(crate) prompt: StaticPrompt,
}

impl PromptEndpoint {
    /// Construct an endpoint over the given reference prompt.
    pub fn new(prompt: StaticPrompt) -> Self {
        Self { prompt }
    }
}
