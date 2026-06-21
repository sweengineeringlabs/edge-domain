#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for Skill trait re-export via skill_svc.rs.

use async_trait::async_trait;
use edge_domain_command::CommandBusBootstrap;
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_domain_observe::StdObserveFactory;
use edge_llm_agent::{Parameter, Skill, SkillMetadata};

struct TestSkill {
    should_fail: bool,
    custom_name: &'static str,
}

#[async_trait]
impl Handler for TestSkill {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        "test_skill"
    }

    async fn execute(&self, req: String, _ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
        if self.should_fail {
            Err(HandlerError::ExecutionFailed("deliberate".to_string()))
        } else {
            Ok(format!("processed: {}", req))
        }
    }
}

impl Skill for TestSkill {
    fn name(&self) -> &str {
        self.custom_name
    }

    fn description(&self) -> &str {
        "A test skill"
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![Parameter {
            name: "input".to_string(),
            description: "Test input".to_string(),
            param_type: "string".to_string(),
            required: true,
        }]
    }

    fn metadata(&self) -> SkillMetadata {
        SkillMetadata {
            name: self.name().to_string(),
            description: self.description().to_string(),
            input_schema: Some("{}".to_string()),
            output_schema: Some("{}".to_string()),
            async_execution: true,
            long_running: false,
        }
    }
}

/// @covers: Skill trait re-export
#[test]
fn test_svc_skill_happy_trait_can_be_implemented() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    assert_eq!(skill.name(), "test");
}

/// @covers: Skill trait re-export — name method
#[test]
fn test_svc_skill_happy_name_returns_configured_value() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "code_review",
    };
    assert_eq!(skill.name(), "code_review");
}

/// @covers: Skill trait re-export — description method
#[test]
fn test_svc_skill_happy_description_returns_configured_value() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    assert_eq!(skill.description(), "A test skill");
}

/// @covers: Skill trait re-export — parameters with values
#[test]
fn test_svc_skill_happy_parameters_returns_list() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    let params = skill.parameters();
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
            _req: String,
            _ctx: HandlerContext<'_>,
        ) -> Result<String, HandlerError> {
            Ok("ok".to_string())
        }
    }

    impl Skill for MinimalSkill {
        fn name(&self) -> &str {
            "minimal"
        }

        fn description(&self) -> &str {
            "Minimal"
        }
    }

    let params = MinimalSkill.parameters();
    assert_eq!(params.len(), 0);
}

/// @covers: Skill trait re-export — metadata method
#[test]
fn test_svc_skill_happy_metadata_returns_skill_metadata() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    let meta = skill.metadata();
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
    let meta = skill.metadata();
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
            _req: String,
            _ctx: HandlerContext<'_>,
        ) -> Result<String, HandlerError> {
            Ok("ok".to_string())
        }
    }

    impl Skill for MinimalSkill {
        fn name(&self) -> &str {
            "minimal"
        }

        fn description(&self) -> &str {
            "Minimal skill"
        }
    }

    let meta = MinimalSkill.metadata();
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
    let security = edge_domain_security::SecurityContext::unauthenticated();
    let bus = edge_domain_command::StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    let result = futures::executor::block_on(skill.execute("input".to_string(), ctx));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "processed: input");
}

/// @covers: Skill trait re-export — execution failure
#[test]
fn test_svc_skill_error_execute_failure_propagates() {
    let skill = TestSkill {
        should_fail: true,
        custom_name: "test",
    };
    let security = edge_domain_security::SecurityContext::unauthenticated();
    let bus = edge_domain_command::StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    let result = futures::executor::block_on(skill.execute("input".to_string(), ctx));
    assert!(result.is_err());
}

/// @covers: Skill trait re-export — Handler id method
#[test]
fn test_svc_skill_happy_implements_handler_contract() {
    let skill = TestSkill {
        should_fail: false,
        custom_name: "test",
    };
    assert_eq!(skill.id(), "test_skill");
}
