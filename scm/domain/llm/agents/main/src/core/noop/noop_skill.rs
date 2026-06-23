//! No-op [`Skill`] implementation for testing the contract.

use crate::api::NoopSkill;
use crate::api::Skill;
use edge_domain_handler::{Handler, HandlerContext, HandlerError};

impl NoopSkill {
    const ID: &'static str = "noop_skill";
    const NAME: &'static str = "noop";
    const DESCRIPTION: &'static str = "No-op skill; implements Skill trait contract for testing";
}

#[async_trait::async_trait]
impl Handler for NoopSkill {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        Self::ID
    }

    async fn execute(
        &self,
        _req: String,
        _ctx: HandlerContext<'_>,
    ) -> Result<String, HandlerError> {
        Err(HandlerError::ExecutionFailed("No-op skill".to_string()))
    }
}

impl Skill for NoopSkill {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn description(&self) -> &str {
        Self::DESCRIPTION
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::CommandBusBootstrap;
    use edge_domain_observer::StdObserveFactory;

    #[test]
    fn test_noop_skill_happy_name_returns_noop() {
        assert_eq!(NoopSkill.name(), "noop");
    }

    #[test]
    fn test_noop_skill_happy_description_returns_string() {
        assert!(!NoopSkill.description().is_empty());
    }

    #[test]
    fn test_noop_skill_error_execute_returns_error() {
        let security = edge_domain_security::SecurityContext::unauthenticated();
        let bus = edge_domain_command::StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
        let result = futures::executor::block_on(NoopSkill.execute("input".to_string(), ctx));
        assert!(result.is_err());
    }
}
