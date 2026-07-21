//! Runnable example: two `Handler`s sharing one constructor-injected `Policy<Input>`, with the
//! per-call value to evaluate carried through `Self::Request` — the generic-per-type port
//! wiring pattern `HandlerContext` structurally can't hold. See issue #149.
//!
//! `Policy::name`/`evaluate` both take `&self` and are fully sync — the simplest port in this
//! series, no lock needed, straightforward `Arc<dyn Policy<...>>` injection like
//! `repository`/`event`/`snapshot`.
//!
//! `CheckWithdrawalHandler::execute` genuinely reads `HandlerContext` for logging. Both handlers
//! here share the same injected `Arc<dyn Policy<Input = u64>>` — a `CompositePolicy` ANDing a
//! minimum and a maximum bound.
//!
//! Run with: `cargo run -p edge-application-policy-handler-example`

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_policy::{
    CompositePolicy, Policy, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse,
    PolicyError,
};
use edge_security_runtime::SecurityContext;

struct MinAmountPolicy(u64);

impl Policy for MinAmountPolicy {
    type Input = u64;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "min-amount" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, u64>) -> Result<(), PolicyError> {
        if *req.input >= self.0 {
            Ok(())
        } else {
            Err(PolicyError::new(
                "min-amount",
                format!("{} is below the minimum of {}", req.input, self.0),
            ))
        }
    }
}

struct MaxAmountPolicy(u64);

impl Policy for MaxAmountPolicy {
    type Input = u64;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "max-amount" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, u64>) -> Result<(), PolicyError> {
        if *req.input <= self.0 {
            Ok(())
        } else {
            Err(PolicyError::new(
                "max-amount",
                format!("{} exceeds the maximum of {}", req.input, self.0),
            ))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct CheckWithdrawalRequest {
    amount: u64,
}
impl edge_application_base::Request for CheckWithdrawalRequest {}

#[derive(Debug, Clone, Copy)]
struct CheckWithdrawalResponse {
    allowed: bool,
}
impl edge_application_base::Response for CheckWithdrawalResponse {}

/// Holds its own injected `Policy`; genuinely reads `req.ctx` inside `execute()`.
struct CheckWithdrawalHandler {
    policy: Arc<dyn Policy<Input = u64>>,
}

#[async_trait]
impl Handler for CheckWithdrawalHandler {
    type Request = CheckWithdrawalRequest;
    type Response = CheckWithdrawalResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, CheckWithdrawalRequest>,
    ) -> Result<CheckWithdrawalResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "check_withdrawal_handler".to_string(),
                message: format!("evaluating withdrawal of {}", req.req.amount),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        println!("  [1] CheckWithdrawalHandler::execute — delegating to its own injected Policy");
        self.policy
            .evaluate(PolicyEvaluateRequest {
                input: &req.req.amount,
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        println!("  [2] CheckWithdrawalHandler::execute — policy allowed the withdrawal");
        Ok(CheckWithdrawalResponse { allowed: true })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct GetPolicyNameRequest;
impl edge_application_base::Request for GetPolicyNameRequest {}

#[derive(Debug, Clone)]
struct GetPolicyNameResponse {
    name: String,
}
impl edge_application_base::Response for GetPolicyNameResponse {}

/// Holds the *same* injected `Policy` instance as `CheckWithdrawalHandler`.
struct GetPolicyNameHandler {
    policy: Arc<dyn Policy<Input = u64>>,
}

#[async_trait]
impl Handler for GetPolicyNameHandler {
    type Request = GetPolicyNameRequest;
    type Response = GetPolicyNameResponse;

    async fn execute(
        &self,
        _req: HandlerExecutionRequest<'_, GetPolicyNameRequest>,
    ) -> Result<GetPolicyNameResponse, HandlerError> {
        println!("  [1] GetPolicyNameHandler::execute — delegating to its own injected Policy");
        let name = self
            .policy
            .name(PolicyNameRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .name;
        println!("  [2] GetPolicyNameHandler::execute — policy reports name = {name:?}");
        Ok(GetPolicyNameResponse {
            name: name.to_string(),
        })
    }
}

#[tokio::main]
async fn main() {
    println!("=== Handler + injected Policy<Input> — constructor injection, Self::Request carries the value ===\n");

    let policy: Arc<dyn Policy<Input = u64>> = Arc::new(
        CompositePolicy::new()
            .with(Box::new(MinAmountPolicy(10)))
            .with(Box::new(MaxAmountPolicy(1000))),
    );
    let withdrawal_handler = CheckWithdrawalHandler {
        policy: Arc::clone(&policy),
    };
    let name_handler = GetPolicyNameHandler {
        policy: Arc::clone(&policy),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] name_handler.execute(GetPolicyNameRequest)");
    let name_resp = name_handler
        .execute(HandlerExecutionRequest {
            req: GetPolicyNameRequest,
            ctx: &ctx,
        })
        .await
        .expect("name should succeed");
    println!("[3] GetPolicyNameHandler reports name = {:?}\n", name_resp.name);

    println!("[0] withdrawal_handler.execute(CheckWithdrawalRequest {{ amount: 500 }})");
    let allowed = withdrawal_handler
        .execute(HandlerExecutionRequest {
            req: CheckWithdrawalRequest { amount: 500 },
            ctx: &ctx,
        })
        .await
        .expect("500 should be within bounds");
    println!("[3] CheckWithdrawalHandler reports allowed = {}\n", allowed.allowed);

    println!("[0] withdrawal_handler.execute(CheckWithdrawalRequest {{ amount: 5 }}) — below minimum");
    let rejected = withdrawal_handler
        .execute(HandlerExecutionRequest {
            req: CheckWithdrawalRequest { amount: 5 },
            ctx: &ctx,
        })
        .await;
    match rejected {
        Ok(_) => println!("[3] unexpectedly succeeded\n"),
        Err(e) => println!("[3] CheckWithdrawalHandler returned an error, as expected: {e}\n"),
    }

    println!("Conclusion: CheckWithdrawalHandler used ctx.observer for real per-request logging, but");
    println!("both handlers reached the actual policy only through their own, independently");
    println!("injected Policy<u64> — never through HandlerContext, which structurally cannot hold a");
    println!("generic-per-type port. The composite policy's own violation propagated cleanly as a");
    println!("real HandlerError.");
}
