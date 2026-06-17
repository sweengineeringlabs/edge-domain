//! Skill trait — named capability that extends Handler.

use crate::api::types::ParameterDocumentationBuilder;
use crate::api::types::{
    ContentPart, InputOutputSchema, MessageContent, ParameterDocumentation, SkillMetadata,
};

use super::parameter::Parameter;
use edge_domain_handler::Handler;

/// A Skill is a named capability an Agent can execute.
///
/// Semantically, a Skill IS-A Handler — it processes requests to produce responses.
/// This allows skills to:
/// - Be invoked directly over ingress/HTTP, gRPC, async queue
/// - Benefit from middleware (auth, rate-limit, cache, trace)
/// - Compose naturally with other domain contracts
pub trait Skill: Handler + Send + Sync {
    /// Skill name (e.g., "code_review", "planning", "memory_retrieve").
    fn name(&self) -> &str;

    /// Human-readable description of what this skill does.
    fn description(&self) -> &str;

    /// Optional: list of input parameters this skill accepts.
    fn parameters(&self) -> Vec<Parameter> {
        vec![]
    }

    /// Get skill metadata including documentation and schemas.
    fn metadata(&self) -> SkillMetadata {
        SkillMetadata {
            name: self.name().to_string(),
            description: self.description().to_string(),
            input_schema: None,
            output_schema: None,
            async_execution: true,
            long_running: false,
        }
    }

    /// Optional structured documentation for each input parameter.
    ///
    /// Defaults to empty; skills override to expose rich parameter docs for
    /// discovery and validation tooling.
    fn parameter_documentation(&self) -> Vec<ParameterDocumentation> {
        vec![]
    }

    /// Optional JSON-Schema describing this skill's input contract.
    ///
    /// Defaults to `None`; skills override to advertise a typed input schema.
    fn input_schema(&self) -> Option<InputOutputSchema> {
        None
    }

    /// Optional JSON-Schema describing this skill's output contract.
    ///
    /// Defaults to `None`; skills override to advertise a typed output schema.
    fn output_schema(&self) -> Option<InputOutputSchema> {
        None
    }

    /// Render the given content into the message body for this skill's prompt.
    ///
    /// The default wraps the supplied parts as multi-modal content; skills that
    /// only emit text override this for a simpler representation.
    fn render_content(&self, parts: Vec<ContentPart>) -> MessageContent {
        MessageContent::parts(parts)
    }

    /// Start a fluent [`ParameterDocumentationBuilder`] for the named parameter.
    fn parameter_documentation_builder(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        param_type: impl Into<String>,
        required: bool,
    ) -> ParameterDocumentationBuilder
    where
        Self: Sized,
    {
        ParameterDocumentationBuilder::new(name, description, param_type, required)
    }
}
