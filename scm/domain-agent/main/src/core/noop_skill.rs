//! No-op Skill implementation for testing the contract.

use crate::api::Skill;
use edge_domain_handler::{Handler, HandlerContext, HandlerError};

/// A no-op skill that implements the Skill trait but performs no work.
/// Used for testing the contract; real implementations live in plugins.
pub(crate) struct NoopSkill;

#[async_trait::async_trait]
impl Handler for NoopSkill {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        "noop_skill"
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
        "noop"
    }

    fn description(&self) -> &str {
        "No-op skill; implements Skill trait"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::CommandBusFactory;

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
        let ctx = HandlerContext {
            security: &security,
            commands: &bus,
        };
        let result = futures::executor::block_on(NoopSkill.execute("input".to_string(), ctx));
        assert!(result.is_err());
    }
}
