//! Basic `Handler` usage example.

use async_trait::async_trait;
use edge_domain_handler::{
    EchoHandler, ExecutionRequest, Handler, HandlerError, IdRequest, IdResponse, PatternRequest,
    PatternResponse,
};

struct Greet;

#[async_trait]
impl Handler for Greet {
    type Request = String;
    type Response = String;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "greet".to_string(),
        })
    }
    fn pattern(&self, _req: PatternRequest) -> Result<PatternResponse, HandlerError> {
        Ok(PatternResponse {
            pattern: "/greet".to_string(),
        })
    }
    async fn execute(&self, req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        let greeting = format!("Hello, {}!", req.req);
        Ok(greeting)
    }
}

fn main() {
    let _echo: EchoHandler<String> = EchoHandler::from(("echo", "/echo"));
    let _greet = Greet;
    println!("handler sub-crate ready");
}
