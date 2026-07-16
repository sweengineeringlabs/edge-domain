//! Basic `Service` usage example.

use edge_application_service::{NameRequest, Service, ServiceError};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct EchoService;

impl Service for EchoService {
    type Request = TextPayload;
    type Response = TextPayload;

    fn execute(&self, req: TextPayload) -> BoxFuture<'_, Result<TextPayload, ServiceError>> {
        Box::pin(async move { Ok(req) })
    }
}

fn main() {
    let service = EchoService;
    let name = service.name(NameRequest).unwrap().name;
    let response = block_on(service.execute(TextPayload("hello".to_string()))).unwrap();
    println!("[{name}] echoed: {}", response.0);
}
