//! `StaticPrompt` — reference [`Prompt`](crate::api::prompt::traits::Prompt) implementation.

use crate::api::prompt::types::{PromptMetadata, RenderContext};

/// Reference prompt template that renders a static body with `{{name}}`
/// placeholders substituted from the [`RenderContext`].
///
/// This is a domain primitive with no external templating engine: it performs
/// deterministic mustache-style substitution so callers can exercise the
/// [`Prompt`](crate::api::prompt::traits::Prompt) contract in tests and wiring.
#[derive(Clone, Debug)]
pub struct StaticPrompt {
    pub(crate) template: String,
    pub(crate) metadata: Option<PromptMetadata>,
}

impl StaticPrompt {
    /// Construct a prompt from a template body and its metadata.
    pub fn new(template: String, metadata: PromptMetadata) -> Self {
        Self { template, metadata: Some(metadata) }
    }

    /// Render the template against `context`, substituting `{{name}}` tokens.
    ///
    /// Returns the rendered string and the list of placeholder names that had
    /// no matching variable in the context (used for completeness checks).
    pub(crate) fn substitute(&self, context: &RenderContext) -> (String, Vec<String>) {
        let variables = self.metadata.as_ref().map(|m| m.variables.as_slice()).unwrap_or(&[]);
        let mut output = self.template.clone();
        let mut missing = Vec::new();
        for var in variables {
            let token = format!("{{{{{}}}}}", var.name);
            match context.get_variable(&var.name) {
                Some(value) => {
                    let rendered = match value {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    output = output.replace(&token, &rendered);
                }
                None if var.required => missing.push(var.name.clone()),
                None => {}
            }
        }
        (output, missing)
    }
}
