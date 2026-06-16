//! Integration tests — `Skill` trait.

use async_trait::async_trait;
use edge_llm_agent::{Parameter, Skill, SkillMetadata};
use edge_domain_command::CommandBusFactory;
use edge_domain_handler::{Handler, HandlerContext, HandlerError};

struct TestSkill {
    should_fail: bool,
}

#[async_trait]
impl Handler for TestSkill {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        "test_skill"
    }

    async fn execute(
        &self,
        req: String,
        _ctx: HandlerContext<'_>,
    ) -> Result<String, HandlerError> {
        if self.should_fail {
            Err(HandlerError::ExecutionFailed("deliberate".to_string()))
        } else {
            Ok(format!("processed: {}", req))
        }
    }
}

impl Skill for TestSkill {
    fn name(&self) -> &str {
        "test"
    }

    fn description(&self) -> &str {
        "A test skill"
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter {
                name: "input".to_string(),
                description: "Test input".to_string(),
                param_type: "string".to_string(),
                required: true,
            },
        ]
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

/// @covers: Skill::name
#[test]
fn test_trait_skill_happy_name_returns_configured_name() {
    let skill = TestSkill {
        should_fail: false,
    };
    assert_eq!(skill.name(), "test");
}

/// @covers: Skill::description
#[test]
fn test_trait_skill_happy_description_returns_configured_description() {
    let skill = TestSkill {
        should_fail: false,
    };
    assert_eq!(skill.description(), "A test skill");
}

/// @covers: Skill::parameters — non-empty
#[test]
fn test_trait_skill_happy_parameters_returns_list() {
    let skill = TestSkill {
        should_fail: false,
    };
    let params = skill.parameters();
    assert_eq!(params.len(), 1);
    assert_eq!(params[0].name, "input");
}

/// @covers: Skill::parameters — structure
#[test]
fn test_trait_skill_happy_parameters_has_correct_structure() {
    let skill = TestSkill {
        should_fail: false,
    };
    let params = skill.parameters();
    assert!(!params.is_empty());
    let param = &params[0];
    assert_eq!(param.param_type, "string");
    assert!(param.required);
}

/// @covers: Skill::metadata — returns configured metadata
#[test]
fn test_trait_skill_happy_metadata_returns_skill_metadata() {
    let skill = TestSkill {
        should_fail: false,
    };
    let meta = skill.metadata();
    assert_eq!(meta.name, "test");
    assert_eq!(meta.description, "A test skill");
}

/// @covers: Skill::metadata — schema fields
#[test]
fn test_trait_skill_happy_metadata_has_schemas() {
    let skill = TestSkill {
        should_fail: false,
    };
    let meta = skill.metadata();
    assert!(meta.input_schema.is_some());
    assert!(meta.output_schema.is_some());
}

/// @covers: Skill::metadata — execution flags
#[test]
fn test_trait_skill_happy_metadata_has_execution_flags() {
    let skill = TestSkill {
        should_fail: false,
    };
    let meta = skill.metadata();
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

    let meta = MinimalSkill.metadata();
    assert_eq!(meta.name, "minimal");
    assert!(meta.async_execution);
}

/// @covers: Skill — extends Handler
#[test]
fn test_trait_skill_happy_implements_handler_contract() {
    let skill = TestSkill {
        should_fail: false,
    };
    assert_eq!(skill.id(), "test_skill");
    assert!(skill.pattern().is_empty()); // default
}

/// @covers: Handler::execute (via Skill)
#[test]
fn test_trait_skill_happy_execute_processes_request() {
    let skill = TestSkill {
        should_fail: false,
    };
    let security = edge_domain_security::SecurityContext::unauthenticated();
    let bus = edge_domain_command::StdCommandBusFactory::direct();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
    };
    let result = futures::executor::block_on(skill.execute("input".to_string(), ctx));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "processed: input");
}

/// @covers: Handler::execute — failure (via Skill)
#[test]
fn test_trait_skill_error_execute_failure_propagates() {
    let skill = TestSkill { should_fail: true };
    let security = edge_domain_security::SecurityContext::unauthenticated();
    let bus = edge_domain_command::StdCommandBusFactory::direct();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
    };
    let result = futures::executor::block_on(skill.execute("input".to_string(), ctx));
    assert!(result.is_err());
}

/// @covers: Skill — all methods together
#[test]
fn test_trait_skill_happy_all_methods_consistent() {
    let skill = TestSkill {
        should_fail: false,
    };
    assert!(!skill.name().is_empty());
    assert!(!skill.description().is_empty());
    let meta = skill.metadata();
    assert_eq!(meta.name, skill.name());
}
