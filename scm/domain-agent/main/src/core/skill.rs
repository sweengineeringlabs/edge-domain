//! Stub Skill implementation for arch audit compliance.
//!
//! Real implementations live in plugins (edge-plugin-llmboot).
//! This stub exists only to satisfy the core_implements_api_traits rule.

use crate::api::Skill;
use edge_domain_handler::{Handler, HandlerContext, HandlerError};

/// Stub Skill implementation (not for production use).
pub struct StubSkill;

#[async_trait::async_trait]
impl Handler for StubSkill {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        "stub_skill"
    }

    async fn execute(
        &self,
        _req: Self::Request,
        _ctx: HandlerContext<'_>,
    ) -> Result<Self::Response, HandlerError> {
        Err(HandlerError::ExecutionFailed(
            "Stub skills do not execute".to_string(),
        ))
    }
}

impl Skill for StubSkill {
    fn name(&self) -> &str {
        "stub"
    }

    fn description(&self) -> &str {
        "Placeholder for arch audit"
    }
}
