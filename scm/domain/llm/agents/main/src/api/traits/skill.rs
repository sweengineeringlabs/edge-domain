//! Skill trait — named capability that extends Handler.

use crate::api::types::{
    InputSchemaRequest, InputSchemaResponse, OutputSchemaRequest, OutputSchemaResponse,
    ParameterDocumentationBuilderRequest, ParameterDocumentationBuilderResponse,
    ParameterDocumentationListRequest, ParameterDocumentationListResponse, RenderContentRequest,
    RenderContentResponse, SkillDescriptionRequest, SkillDescriptionResponse,
    SkillMetadataLookupRequest, SkillMetadataLookupResponse, SkillNameRequest, SkillNameResponse,
    SkillParametersRequest, SkillParametersResponse,
};
use crate::api::AgentError;

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
    fn name(&self, req: SkillNameRequest) -> Result<SkillNameResponse, AgentError>;

    /// Human-readable description of what this skill does.
    fn description(
        &self,
        req: SkillDescriptionRequest,
    ) -> Result<SkillDescriptionResponse, AgentError>;

    /// Optional: list of input parameters this skill accepts.
    fn parameters(
        &self,
        _req: SkillParametersRequest,
    ) -> Result<SkillParametersResponse, AgentError> {
        Ok(SkillParametersResponse { parameters: vec![] })
    }

    /// Get skill metadata including documentation and schemas.
    fn metadata(
        &self,
        _req: SkillMetadataLookupRequest,
    ) -> Result<SkillMetadataLookupResponse, AgentError> {
        Ok(SkillMetadataLookupResponse {
            metadata: Box::new(crate::api::types::SkillMetadata {
                name: self.name(SkillNameRequest)?.name,
                description: self.description(SkillDescriptionRequest)?.description,
                input_schema: None,
                output_schema: None,
                async_execution: true,
                long_running: false,
            }),
        })
    }

    /// Optional structured documentation for each input parameter.
    ///
    /// Defaults to empty; skills override to expose rich parameter docs for
    /// discovery and validation tooling.
    fn parameter_documentation(
        &self,
        _req: ParameterDocumentationListRequest,
    ) -> Result<ParameterDocumentationListResponse, AgentError> {
        Ok(ParameterDocumentationListResponse {
            documentation: vec![],
        })
    }

    /// Optional JSON-Schema describing this skill's input contract.
    ///
    /// Defaults to `None`; skills override to advertise a typed input schema.
    fn input_schema(&self, _req: InputSchemaRequest) -> Result<InputSchemaResponse, AgentError> {
        Ok(InputSchemaResponse { schema: None })
    }

    /// Optional JSON-Schema describing this skill's output contract.
    ///
    /// Defaults to `None`; skills override to advertise a typed output schema.
    fn output_schema(&self, _req: OutputSchemaRequest) -> Result<OutputSchemaResponse, AgentError> {
        Ok(OutputSchemaResponse { schema: None })
    }

    /// Render the given content into the message body for this skill's prompt.
    ///
    /// The default wraps the supplied parts as multi-modal content; skills that
    /// only emit text override this for a simpler representation.
    fn render_content(
        &self,
        req: RenderContentRequest,
    ) -> Result<RenderContentResponse, AgentError> {
        Ok(RenderContentResponse {
            content: Box::new(crate::api::types::MessageContent::parts(req.parts)),
        })
    }

    /// Start a fluent [`ParameterDocumentationBuilder`](crate::api::types::ParameterDocumentationBuilder) for the named parameter.
    fn parameter_documentation_builder(
        &self,
        req: ParameterDocumentationBuilderRequest,
    ) -> Result<ParameterDocumentationBuilderResponse, AgentError> {
        Ok(ParameterDocumentationBuilderResponse {
            builder: Box::new(crate::api::types::ParameterDocumentationBuilder::new(
                req.name,
                req.description,
                req.param_type,
                req.required,
            )),
        })
    }
}
