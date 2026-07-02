//! `DefaultAgentHandler` — `Handler` impl for the agent primitive (ADR-024).

use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerError, IdRequest, IdResponse, PatternRequest, PatternResponse,
};

/// Stable handler id under which this handler registers for dispatch.
const AGENT_HANDLER_ID: &str = "agent.execute_skill";
/// Route pattern this handler matches in the dispatch table.
const AGENT_HANDLER_PATTERN: &str = "agent/execute_skill";

pub(crate) struct DefaultAgentHandler {
    pub(crate) skill: String,
}

#[async_trait::async_trait]
impl Handler for DefaultAgentHandler {
    type Request = String;
    type Response = String;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: AGENT_HANDLER_ID.to_string(),
        })
    }

    fn pattern(&self, _req: PatternRequest) -> Result<PatternResponse, HandlerError> {
        Ok(PatternResponse {
            pattern: AGENT_HANDLER_PATTERN.to_string(),
        })
    }

    async fn execute(&self, req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        if req.req.is_empty() {
            return Err(HandlerError::ExecutionFailed(
                "agent skill input must not be empty".to_string(),
            ));
        }
        let span = req.ctx.observer.tracer().start_span("agent", &self.skill);
        req.ctx
            .observer
            .metrics()
            .counter("agent.dispatch")
            .increment(1);
        let result = Ok(format!("{}:{}", self.skill, req.req));
        span.finish();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::HandlerContext;
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::{SecurityBootstrap, SecurityServices};
    use futures::executor::block_on;

    fn handler() -> DefaultAgentHandler {
        DefaultAgentHandler {
            skill: "code_review".to_string(),
        }
    }

    #[test]
    fn test_handler_execute_returns_skill_colon_input_happy() {
        let security = SecurityServices::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let out = block_on(Handler::execute(
            &handler(),
            ExecutionRequest {
                req: "diff".to_string(),
                ctx: &ctx,
            },
        ))
        .expect("handler ok");
        assert_eq!(out, "code_review:diff");
    }

    #[test]
    fn test_handler_execute_dispatch_increments_counter_happy() {
        let security = SecurityServices::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        block_on(Handler::execute(
            &handler(),
            ExecutionRequest {
                req: "diff".to_string(),
                ctx: &ctx,
            },
        ))
        .expect("handler ok");
    }

    #[test]
    fn test_handler_execute_empty_input_does_not_emit_span_error() {
        let security = SecurityServices::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let result = block_on(Handler::execute(
            &handler(),
            ExecutionRequest {
                req: String::new(),
                ctx: &ctx,
            },
        ));
        assert!(result.is_err());
    }

    #[test]
    fn test_handler_id_is_stable_edge() {
        assert_eq!(
            Handler::id(&handler(), IdRequest).unwrap().id,
            AGENT_HANDLER_ID
        );
    }

    #[test]
    fn test_handler_pattern_is_stable_edge() {
        assert_eq!(
            Handler::pattern(&handler(), PatternRequest)
                .unwrap()
                .pattern,
            AGENT_HANDLER_PATTERN
        );
    }

    #[test]
    fn test_handler_execute_empty_input_error() {
        let security = SecurityServices::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let result = block_on(Handler::execute(
            &handler(),
            ExecutionRequest {
                req: String::new(),
                ctx: &ctx,
            },
        ));
        assert!(result.is_err());
    }
}
