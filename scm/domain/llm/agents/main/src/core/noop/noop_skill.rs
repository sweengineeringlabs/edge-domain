//! No-op [`Skill`] implementation for testing the contract.

use crate::api::NoopSkill;
use crate::api::Skill;
use crate::api::{
    SkillDescriptionRequest, SkillDescriptionResponse, SkillNameRequest, SkillNameResponse,
};
use edge_domain_handler::{ExecutionRequest, Handler, HandlerError, IdRequest, IdResponse};

impl NoopSkill {
    const ID: &'static str = "noop_skill";
    const NAME: &'static str = "noop";
    const DESCRIPTION: &'static str = "No-op skill; implements Skill trait contract for testing";
}

#[async_trait::async_trait]
impl Handler for NoopSkill {
    type Request = String;
    type Response = String;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: Self::ID.to_string(),
        })
    }

    async fn execute(&self, _req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        Err(HandlerError::ExecutionFailed("No-op skill".to_string()))
    }
}

impl Skill for NoopSkill {
    fn name(&self, _req: SkillNameRequest) -> Result<SkillNameResponse, crate::api::AgentError> {
        Ok(SkillNameResponse {
            name: Self::NAME.to_string(),
        })
    }

    fn description(
        &self,
        _req: SkillDescriptionRequest,
    ) -> Result<SkillDescriptionResponse, crate::api::AgentError> {
        Ok(SkillDescriptionResponse {
            description: Self::DESCRIPTION.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_observer::StdObserveFactory;

    #[test]
    fn test_noop_skill_happy_name_returns_noop() {
        assert_eq!(NoopSkill.name(SkillNameRequest).unwrap().name, "noop");
    }

    #[test]
    fn test_noop_skill_happy_description_returns_string() {
        assert!(!NoopSkill
            .description(SkillDescriptionRequest)
            .unwrap()
            .description
            .is_empty());
    }

    #[test]
    fn test_noop_skill_error_execute_returns_error() {
        
        let security = edge_security_runtime::SecurityContext::unauthenticated();
        let bus = edge_domain_command::DirectCommandBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = edge_domain_handler::HandlerContext {
            security: &security,
            commands: &bus,
            observer: observer.as_ref(),
        };
        let result = futures::executor::block_on(NoopSkill.execute(ExecutionRequest {
            req: "input".to_string(),
            ctx: &ctx,
        }));
        assert!(result.is_err());
    }
}
