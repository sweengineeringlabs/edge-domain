//! Basic `Handler` usage example.

use async_trait::async_trait;
use edge_domain_handler::{Handler, HandlerContext, HandlerError, HandlerProvider};

struct Handlers;
impl HandlerProvider for Handlers {}

struct Greet;

#[async_trait]
impl Handler for Greet {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        "greet"
    }
    fn pattern(&self) -> &str {
        "/greet"
    }
    async fn execute(&self, req: String, ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
        let greeting = format!("Hello, {}!", req);
        // ctx provides request-scoped context like security, tracing, etc.
        Ok(greeting)
    }
}

fn main() {
    let _echo = Handlers::echo_handler("echo", "/echo");
    let _greet = Greet;
    println!("handler sub-crate ready");
}
