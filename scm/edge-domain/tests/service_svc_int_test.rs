#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Service trait is exported from the crate root.
#![cfg(feature = "service")]

use edge_application::Service;
use edge_application_service::{NameRequest, NameResponse, ServiceError};

struct Echo;
impl Service for Echo {
    type Request = String;
    type Response = String;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "echo".to_string(),
        })
    }
    fn execute(
        &self,
        input: String,
    ) -> futures::future::BoxFuture<'_, Result<String, ServiceError>> {
        Box::pin(async move { Ok(input) })
    }
}

#[tokio::test]
async fn test_service_svc_facade_execute_returns_input() {
    assert_eq!(Echo.execute("hi".into()).await.unwrap(), "hi");
}

#[test]
fn test_service_svc_facade_name_is_stable() {
    assert_eq!(Echo.name(NameRequest).unwrap().name, "echo");
}
