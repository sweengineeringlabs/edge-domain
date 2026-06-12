//! Basic `Service` usage example.

use edge_domain_service::{Service, ServiceError};
use futures::future::BoxFuture;

struct EchoService;

impl Service<String, String> for EchoService {
    fn name(&self) -> &str { "echo" }
    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

fn main() {
    println!("service sub-crate ready");
}
