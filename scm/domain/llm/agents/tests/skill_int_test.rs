#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests — `Skill` trait.

use async_trait::async_trait;
use edge_domain_command::CommandBusBootstrap;
use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerContext, HandlerError, IdRequest, IdResponse, PatternRequest,
};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::{SecurityBootstrap, SecurityServices};
use edge_llm_agent::{
    AgentError, ContentPart, InputSchemaRequest, MessageContent, OutputSchemaRequest, Parameter,
    ParameterDocumentationBuilderRequest, ParameterDocumentationListRequest, RenderContentRequest,
    Skill, SkillDescriptionRequest, SkillDescriptionResponse, SkillMetadata,
    SkillMetadataLookupRequest, SkillMetadataLookupResponse, SkillNameRequest, SkillNameResponse,
    SkillParametersRequest, SkillParametersResponse,
};

struct TestSkill {
    should_fail: bool,
}

#[async_trait]
impl Handler for TestSkill {
    type Request = String;
    type Response = String;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "test_skill".to_string(),
        })
    }

    async fn execute(&self, req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        if self.should_fail {
            Err(HandlerError::ExecutionFailed("deliberate".to_string()))
        } else {
            Ok(format!("processed: {}", req.req))
        }
    }
}

impl Skill for TestSkill {
    fn name(&self, _req: SkillNameRequest) -> Result<SkillNameResponse, AgentError> {
        Ok(SkillNameResponse {
            name: "test".to_string(),
        })
    }

    fn description(
        &self,
        _req: SkillDescriptionRequest,
    ) -> Result<SkillDescriptionResponse, AgentError> {
        Ok(SkillDescriptionResponse {
            description: "A test skill".to_string(),
        })
    }

    fn parameters(
        &self,
        _req: SkillParametersRequest,
    ) -> Result<SkillParametersResponse, AgentError> {
        Ok(SkillParametersResponse {
            parameters: vec![Parameter {
                name: "input".to_string(),
                description: "Test input".to_string(),
                param_type: "string".to_string(),
                required: true,
            }],
        })
    }

    fn metadata(
        &self,
        _req: SkillMetadataLookupRequest,
    ) -> Result<SkillMetadataLookupResponse, AgentError> {
        Ok(SkillMetadataLookupResponse {
            metadata: Box::new(SkillMetadata {
                name: self.name(SkillNameRequest)?.name,
                description: self.description(SkillDescriptionRequest)?.description,
                input_schema: Some("{}".to_string()),
                output_schema: Some("{}".to_string()),
                async_execution: true,
                long_running: false,
            }),
        })
    }
}

/// @covers: Skill::name
#[test]
fn test_trait_skill_happy_name_returns_configured_name() {
    let skill = TestSkill { should_fail: false };
    assert_eq!(skill.name(SkillNameRequest).unwrap().name, "test");
}

/// @covers: Skill::description
#[test]
fn test_trait_skill_happy_description_returns_configured_description() {
    let skill = TestSkill { should_fail: false };
    assert_eq!(
        skill
            .description(SkillDescriptionRequest)
            .unwrap()
            .description,
        "A test skill"
    );
}

/// @covers: Skill::parameters — non-empty
#[test]
fn test_trait_skill_happy_parameters_returns_list() {
    let skill = TestSkill { should_fail: false };
    let params = skill.parameters(SkillParametersRequest).unwrap().parameters;
    assert_eq!(params.len(), 1);
    assert_eq!(params[0].name, "input");
}

/// @covers: Skill::parameters — structure
#[test]
fn test_trait_skill_happy_parameters_has_correct_structure() {
    let skill = TestSkill { should_fail: false };
    let params = skill.parameters(SkillParametersRequest).unwrap().parameters;
    assert!(!params.is_empty());
    let param = &params[0];
    assert_eq!(param.param_type, "string");
    assert!(param.required);
}

/// @covers: Skill::metadata — returns configured metadata
#[test]
fn test_trait_skill_happy_metadata_returns_skill_metadata() {
    let skill = TestSkill { should_fail: false };
    let meta = skill.metadata(SkillMetadataLookupRequest).unwrap().metadata;
    assert_eq!(meta.name, "test");
    assert_eq!(meta.description, "A test skill");
}

/// @covers: Skill::metadata — schema fields
#[test]
fn test_trait_skill_happy_metadata_has_schemas() {
    let skill = TestSkill { should_fail: false };
    let meta = skill.metadata(SkillMetadataLookupRequest).unwrap().metadata;
    assert!(meta.input_schema.is_some());
    assert!(meta.output_schema.is_some());
}

/// @covers: Skill::metadata — execution flags
#[test]
fn test_trait_skill_happy_metadata_has_execution_flags() {
    let skill = TestSkill { should_fail: false };
    let meta = skill.metadata(SkillMetadataLookupRequest).unwrap().metadata;
    assert!(meta.async_execution);
    assert!(!meta.long_running);
}

/// @covers: Skill::metadata — default implementation
#[test]
fn test_trait_skill_edge_metadata_default_returns_skill_metadata() {
    struct MinimalSkill;

    #[async_trait]
    impl Handler for MinimalSkill {
        type Request = String;
        type Response = String;

        async fn execute(
            &self,
            _req: ExecutionRequest<'_, String>,
        ) -> Result<String, HandlerError> {
            Ok("ok".to_string())
        }
    }

    impl Skill for MinimalSkill {
        fn name(&self, _req: SkillNameRequest) -> Result<SkillNameResponse, AgentError> {
            Ok(SkillNameResponse {
                name: "minimal".to_string(),
            })
        }

        fn description(
            &self,
            _req: SkillDescriptionRequest,
        ) -> Result<SkillDescriptionResponse, AgentError> {
            Ok(SkillDescriptionResponse {
                description: "Minimal".to_string(),
            })
        }
    }

    let meta = MinimalSkill
        .metadata(SkillMetadataLookupRequest)
        .unwrap()
        .metadata;
    assert_eq!(meta.name, "minimal");
    assert!(meta.async_execution);
}

/// @covers: Skill — extends Handler
#[test]
fn test_trait_skill_happy_implements_handler_contract() {
    let skill = TestSkill { should_fail: false };
    assert_eq!(skill.id(IdRequest).unwrap().id, "test_skill");
    assert!(skill.pattern(PatternRequest).unwrap().pattern.is_empty()); // default
}

/// @covers: Handler::execute (via Skill)
#[test]
fn test_trait_skill_happy_execute_processes_request() {
    let skill = TestSkill { should_fail: false };
    let security = SecurityServices::unauthenticated();
    let bus = edge_domain_command::StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(skill.execute(ExecutionRequest {
        req: "input".to_string(),
        ctx: &ctx,
    }));
    assert_eq!(result.unwrap(), "processed: input");
}

/// @covers: Handler::execute — failure (via Skill)
#[test]
fn test_trait_skill_error_execute_failure_propagates() {
    let skill = TestSkill { should_fail: true };
    let security = SecurityServices::unauthenticated();
    let bus = edge_domain_command::StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(skill.execute(ExecutionRequest {
        req: "input".to_string(),
        ctx: &ctx,
    }));
    assert!(result.is_err());
}

/// @covers: Skill — all methods together
#[test]
fn test_trait_skill_happy_all_methods_consistent() {
    let skill = TestSkill { should_fail: false };
    assert!(!skill.name(SkillNameRequest).unwrap().name.is_empty());
    assert!(!skill
        .description(SkillDescriptionRequest)
        .unwrap()
        .description
        .is_empty());
    let meta = skill.metadata(SkillMetadataLookupRequest).unwrap().metadata;
    assert_eq!(meta.name, skill.name(SkillNameRequest).unwrap().name);
}

fn sample_skill() -> TestSkill {
    TestSkill { should_fail: false }
}

// --- parameter_documentation ---

/// @covers: parameter_documentation
#[test]
fn test_parameter_documentation_defaults_empty_happy() {
    assert!(sample_skill()
        .parameter_documentation(ParameterDocumentationListRequest)
        .unwrap()
        .documentation
        .is_empty());
}

/// @covers: parameter_documentation
#[test]
fn test_parameter_documentation_no_required_docs_error() {
    // Default skills expose no structured docs even when they declare parameters.
    let skill = sample_skill();
    assert!(!skill
        .parameters(SkillParametersRequest)
        .unwrap()
        .parameters
        .is_empty());
    assert!(skill
        .parameter_documentation(ParameterDocumentationListRequest)
        .unwrap()
        .documentation
        .is_empty());
}

/// @covers: parameter_documentation
#[test]
fn test_parameter_documentation_override_edge() {
    let skill = sample_skill();
    let built = skill
        .parameter_documentation_builder(ParameterDocumentationBuilderRequest {
            name: "q".to_string(),
            description: "query".to_string(),
            param_type: "string".to_string(),
            required: true,
        })
        .unwrap()
        .builder
        .build();
    assert_eq!(built.name, "q");
    assert!(skill
        .parameter_documentation(ParameterDocumentationListRequest)
        .unwrap()
        .documentation
        .is_empty());
}

// --- input_schema ---

/// @covers: input_schema
#[test]
fn test_input_schema_defaults_none_happy() {
    assert!(sample_skill()
        .input_schema(InputSchemaRequest)
        .unwrap()
        .schema
        .is_none());
}

/// @covers: input_schema
#[test]
fn test_input_schema_distinct_from_metadata_error() {
    // The default `input_schema` getter is None even though `metadata` sets one.
    let skill = sample_skill();
    assert!(skill
        .metadata(SkillMetadataLookupRequest)
        .unwrap()
        .metadata
        .input_schema
        .is_some());
    assert!(skill
        .input_schema(InputSchemaRequest)
        .unwrap()
        .schema
        .is_none());
}

/// @covers: input_schema
#[test]
fn test_input_schema_minimal_skill_edge() {
    struct MinimalSkill;
    #[async_trait]
    impl Handler for MinimalSkill {
        type Request = String;
        type Response = String;
        async fn execute(
            &self,
            _req: ExecutionRequest<'_, String>,
        ) -> Result<String, HandlerError> {
            Ok("ok".to_string())
        }
    }
    impl Skill for MinimalSkill {
        fn name(&self, _req: SkillNameRequest) -> Result<SkillNameResponse, AgentError> {
            Ok(SkillNameResponse {
                name: "minimal".to_string(),
            })
        }
        fn description(
            &self,
            _req: SkillDescriptionRequest,
        ) -> Result<SkillDescriptionResponse, AgentError> {
            Ok(SkillDescriptionResponse {
                description: "Minimal".to_string(),
            })
        }
    }
    assert!(MinimalSkill
        .input_schema(InputSchemaRequest)
        .unwrap()
        .schema
        .is_none());
}

// --- output_schema ---

/// @covers: output_schema
#[test]
fn test_output_schema_defaults_none_happy() {
    assert!(sample_skill()
        .output_schema(OutputSchemaRequest)
        .unwrap()
        .schema
        .is_none());
}

/// @covers: output_schema
#[test]
fn test_output_schema_distinct_from_metadata_error() {
    let skill = sample_skill();
    assert!(skill
        .metadata(SkillMetadataLookupRequest)
        .unwrap()
        .metadata
        .output_schema
        .is_some());
    assert!(skill
        .output_schema(OutputSchemaRequest)
        .unwrap()
        .schema
        .is_none());
}

/// @covers: output_schema
#[test]
fn test_output_schema_independent_of_input_edge() {
    let skill = sample_skill();
    assert_eq!(
        skill
            .input_schema(InputSchemaRequest)
            .unwrap()
            .schema
            .is_none(),
        skill
            .output_schema(OutputSchemaRequest)
            .unwrap()
            .schema
            .is_none()
    );
}

// --- render_content ---

/// @covers: render_content
#[test]
fn test_render_content_wraps_parts_happy() {
    let skill = sample_skill();
    let content = *skill
        .render_content(RenderContentRequest {
            parts: vec![ContentPart::text("hello")],
        })
        .unwrap()
        .content;
    assert!(matches!(content, MessageContent::Parts(_)));
}

/// @covers: render_content
#[test]
fn test_render_content_empty_parts_error() {
    let skill = sample_skill();
    let content = *skill
        .render_content(RenderContentRequest { parts: vec![] })
        .unwrap()
        .content;
    match content {
        MessageContent::Parts(parts) => assert!(parts.is_empty()),
        MessageContent::Text(_) => panic!("expected Parts variant"),
    }
}

/// @covers: render_content
#[test]
fn test_render_content_multiple_parts_edge() {
    let skill = sample_skill();
    let content = *skill
        .render_content(RenderContentRequest {
            parts: vec![
                ContentPart::text("a"),
                ContentPart::image_url("http://x/y.png"),
            ],
        })
        .unwrap()
        .content;
    match content {
        MessageContent::Parts(parts) => assert_eq!(parts.len(), 2),
        MessageContent::Text(_) => panic!("expected Parts variant"),
    }
}

// --- parameter_documentation_builder ---

/// @covers: parameter_documentation_builder
#[test]
fn test_parameter_documentation_builder_sets_required_fields_happy() {
    let doc = sample_skill()
        .parameter_documentation_builder(ParameterDocumentationBuilderRequest {
            name: "name".to_string(),
            description: "the name".to_string(),
            param_type: "string".to_string(),
            required: true,
        })
        .unwrap()
        .builder
        .build();
    assert_eq!(doc.name, "name");
    assert!(doc.required);
}

/// @covers: parameter_documentation_builder
#[test]
fn test_parameter_documentation_builder_optional_defaults_none_error() {
    let doc = sample_skill()
        .parameter_documentation_builder(ParameterDocumentationBuilderRequest {
            name: "name".to_string(),
            description: "d".to_string(),
            param_type: "string".to_string(),
            required: false,
        })
        .unwrap()
        .builder
        .build();
    assert!(doc.default.is_none());
    assert!(doc.validation_rules.is_none());
}

/// @covers: parameter_documentation_builder
#[test]
fn test_parameter_documentation_builder_examples_accumulate_edge() {
    let doc = sample_skill()
        .parameter_documentation_builder(ParameterDocumentationBuilderRequest {
            name: "name".to_string(),
            description: "d".to_string(),
            param_type: "string".to_string(),
            required: true,
        })
        .unwrap()
        .builder
        .example(serde_json::json!("a"))
        .example(serde_json::json!("b"))
        .build();
    assert_eq!(doc.examples.len(), 2);
}
