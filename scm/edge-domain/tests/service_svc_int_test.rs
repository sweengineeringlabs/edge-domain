#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Service trait is exported from the crate root.
#![cfg(feature = "service")]

use edge_application::Service;
use edge_application_service::{NameRequest, NameResponse, ServiceError};

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct Echo;
impl Service for Echo {
    type Request = TextPayload;
    type Response = TextPayload;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "echo".to_string(),
        })
    }
    fn execute(
        &self,
        input: TextPayload,
    ) -> futures::future::BoxFuture<'_, Result<TextPayload, ServiceError>> {
        Box::pin(async move { Ok(input) })
    }
}

#[tokio::test]
async fn test_service_svc_facade_execute_returns_input() {
    assert_eq!(
        Echo.execute(TextPayload("hi".into())).await.unwrap(),
        TextPayload("hi".into())
    );
}

#[test]
fn test_service_svc_facade_name_is_stable() {
    assert_eq!(Echo.name(NameRequest).unwrap().name, "echo");
}
