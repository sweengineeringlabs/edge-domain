//! Basic `Service` usage example.

use edge_application_service::{NameRequest, Service, ServiceError};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct EchoService;

impl Service for EchoService {
    type Request = String;
    type Response = String;

    fn execute(&self, req: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

fn main() {
    let service = EchoService;
    let name = service.name(NameRequest).unwrap().name;
    let response = block_on(service.execute("hello".to_string())).unwrap();
    println!("[{name}] echoed: {response}");
}
