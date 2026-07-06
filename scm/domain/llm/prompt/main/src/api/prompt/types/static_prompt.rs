//! `StaticPrompt` — reference [`Prompt`](crate::api::prompt::traits::Prompt) implementation.

use crate::api::prompt::types::PromptMetadata;

/// Reference prompt template that renders a static body with `{{name}}`
/// placeholders substituted from the [`RenderContext`](crate::api::prompt::types::RenderContext).
///
/// This is a domain primitive with no external templating engine: it performs
/// deterministic mustache-style substitution so callers can exercise the
/// [`Prompt`](crate::api::prompt::traits::Prompt) contract in tests and wiring.
#[derive(Clone, Debug)]
pub struct StaticPrompt {
    pub(crate) template: String,
    pub(crate) metadata: Option<PromptMetadata>,
}
