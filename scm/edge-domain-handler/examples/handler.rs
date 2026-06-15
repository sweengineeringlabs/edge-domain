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
    async fn execute(&self, req: String, _ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
        Ok(format!("Hello, {}!", req))
    }
}

fn main() {
    let _echo = Handlers::echo_handler("echo", "/echo");
    println!("handler sub-crate ready");
}
