#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for Skill trait re-export via skill_svc.rs.

use async_trait::async_trait;
use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerContext, HandlerError, IdRequest, IdResponse,
};
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use edge_llm_agent::{
    AgentError, Parameter, Skill, SkillDescriptionRequest, SkillDescriptionResponse, SkillMetadata,
    SkillMetadataLookupRequest, SkillMetadataLookupResponse, SkillNameRequest, SkillNameResponse,
    SkillParametersRequest, SkillParametersResponse,
};

struct TestSkill {
    should_fail: bool,
    custom_name: &'static str,
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
            name: self.custom_name.to_string(),
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

/// @covers: Skill trait re-export
#[test]
fn test_svc_skill_happy_trait_can_be_implemented() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    assert_eq!(skill.name(SkillNameRequest).unwrap().name, "test");
}

/// @covers: Skill trait re-export — name method
#[test]
fn test_svc_skill_happy_name_returns_configured_value() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "code_review",
    };
    assert_eq!(skill.name(SkillNameRequest).unwrap().name, "code_review");
}

/// @covers: Skill trait re-export — description method
#[test]
fn test_svc_skill_happy_description_returns_configured_value() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    assert_eq!(
        skill
            .description(SkillDescriptionRequest)
            .unwrap()
            .description,
        "A test skill"
    );
}

/// @covers: Skill trait re-export — parameters with values
#[test]
fn test_svc_skill_happy_parameters_returns_list() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    let params = skill.parameters(SkillParametersRequest).unwrap().parameters;
    assert_eq!(params.len(), 1);
    assert_eq!(params[0].name, "input");
    assert!(params[0].required);
}

/// @covers: Skill trait re-export — parameters empty
#[test]
fn test_svc_skill_edge_parameters_can_be_empty() {
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

    let params = MinimalSkill
        .parameters(SkillParametersRequest)
        .unwrap()
        .parameters;
    assert_eq!(params.len(), 0);
}

/// @covers: Skill trait re-export — metadata method
#[test]
fn test_svc_skill_happy_metadata_returns_skill_metadata() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    let meta = skill.metadata(SkillMetadataLookupRequest).unwrap().metadata;
    assert_eq!(meta.name, "test");
    assert_eq!(meta.description, "A test skill");
}

/// @covers: Skill trait re-export — metadata with schemas
#[test]
fn test_svc_skill_happy_metadata_has_optional_schemas() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    let meta = skill.metadata(SkillMetadataLookupRequest).unwrap().metadata;
    assert!(meta.input_schema.is_some());
    assert!(meta.output_schema.is_some());
    assert_eq!(meta.input_schema.unwrap(), "{}");
}

/// @covers: Skill trait re-export — metadata default implementation
#[test]
fn test_svc_skill_edge_metadata_default_no_schemas() {
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
                description: "Minimal skill".to_string(),
            })
        }
    }

    let meta = MinimalSkill
        .metadata(SkillMetadataLookupRequest)
        .unwrap()
        .metadata;
    assert_eq!(meta.name, "minimal");
    assert!(meta.async_execution);
    assert_eq!(meta.input_schema, None);
    assert_eq!(meta.output_schema, None);
}

/// @covers: Skill trait re-export — execution via Handler
#[test]
fn test_svc_skill_happy_execute_processes_request() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    let security = SecurityContext::unauthenticated();
    let bus = edge_domain_command::DirectCommandBus;
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

/// @covers: Skill trait re-export — execution failure
#[test]
fn test_svc_skill_error_execute_failure_propagates() {
    let skill = TestSkill {
        should_fail: true,
        custom_name: "test",
    };
    let security = SecurityContext::unauthenticated();
    let bus = edge_domain_command::DirectCommandBus;
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

/// @covers: Skill trait re-export — Handler id method
#[test]
fn test_svc_skill_happy_implements_handler_contract() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    assert_eq!(skill.id(IdRequest).unwrap().id, "test_skill");
}
