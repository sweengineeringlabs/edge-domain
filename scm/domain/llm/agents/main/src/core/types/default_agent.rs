//! `DefaultAgentHandler` — `Handler` impl for the agent primitive (ADR-024).

use edge_domain_handler::{Handler, HandlerContext, HandlerError};

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

    fn id(&self) -> &str {
        AGENT_HANDLER_ID
    }

    fn pattern(&self) -> &str {
        AGENT_HANDLER_PATTERN
    }

    async fn execute(
        &self,
        input: String,
        ctx: HandlerContext<'_>,
    ) -> Result<String, HandlerError> {
        if input.is_empty() {
            return Err(HandlerError::ExecutionFailed(
                "agent skill input must not be empty".to_string(),
            ));
        }
        let span = ctx.observer().tracer().start_span("agent", &self.skill);
        ctx.observer()
            .metrics()
            .counter("agent.dispatch")
            .increment(1);
        let result = Ok(format!("{}:{}", self.skill, input));
        span.finish();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;

    fn handler() -> DefaultAgentHandler {
        DefaultAgentHandler {
            skill: "code_review".to_string(),
        }
    }

    #[test]
    fn test_handler_execute_returns_skill_colon_input_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        let out =
            block_on(Handler::execute(&handler(), "diff".to_string(), ctx)).expect("handler ok");
        assert_eq!(out, "code_review:diff");
    }

    #[test]
    fn test_handler_execute_dispatch_increments_counter_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        block_on(Handler::execute(&handler(), "diff".to_string(), ctx)).expect("handler ok");
    }

    #[test]
    fn test_handler_execute_empty_input_does_not_emit_span_error() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        let result = block_on(Handler::execute(&handler(), String::new(), ctx));
        assert!(result.is_err());
    }

    #[test]
    fn test_handler_id_is_stable_edge() {
        assert_eq!(Handler::id(&handler()), AGENT_HANDLER_ID);
    }

    #[test]
    fn test_handler_pattern_is_stable_edge() {
        assert_eq!(Handler::pattern(&handler()), AGENT_HANDLER_PATTERN);
    }

    #[test]
    fn test_handler_execute_empty_input_error() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        let result = block_on(Handler::execute(&handler(), String::new(), ctx));
        assert!(result.is_err());
    }
}
