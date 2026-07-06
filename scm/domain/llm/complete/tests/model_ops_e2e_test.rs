//! Scenario coverage for the `ModelOps` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use async_trait::async_trait;
use edge_llm_complete::{CompleteError, ModelInfo, ModelInfoRequest, ModelInfoResponse, ModelOps};
use futures::executor::block_on;

struct EchoModelOps;

#[async_trait]
impl ModelOps for EchoModelOps {
    async fn find_model(
        &self,
        req: ModelInfoRequest<'_>,
    ) -> Result<ModelInfoResponse, CompleteError> {
        if req.model == "echo" {
            Ok(ModelInfoResponse {
                info: Box::new(ModelInfo::new("echo", "Echo Model", "echo", 4096)),
            })
        } else {
            Err(CompleteError::ModelNotFound(req.model.to_string()))
        }
    }
}

// ── find_model ────────────────────────────────────────────────────────────────

#[test]
fn test_find_model_known_model_returns_info_happy() {
    let resp = block_on(EchoModelOps.find_model(ModelInfoRequest { model: "echo" })).unwrap();
    assert_eq!(resp.info.id, "echo");
}

#[test]
fn test_find_model_unknown_model_returns_error_error() {
    let err = block_on(EchoModelOps.find_model(ModelInfoRequest { model: "gpt-999" })).unwrap_err();
    assert!(matches!(err, CompleteError::ModelNotFound(_)));
}

#[test]
fn test_find_model_empty_name_returns_error_edge() {
    let err = block_on(EchoModelOps.find_model(ModelInfoRequest { model: "" })).unwrap_err();
    assert!(matches!(err, CompleteError::ModelNotFound(_)));
}

// ── create_model_info ─────────────────────────────────────────────────────────

#[test]
fn test_create_model_info_sets_fields_happy() {
    let m = EchoModelOps::create_model_info("gpt-4", "GPT-4", "openai", 128_000);
    assert_eq!(m.id, "gpt-4");
    assert_eq!(m.context_window, 128_000);
}

#[test]
fn test_create_model_info_empty_id_is_valid_error() {
    let m = EchoModelOps::create_model_info("", "", "", 0);
    assert!(m.id.is_empty());
}

#[test]
fn test_create_model_info_capabilities_default_false_edge() {
    let m = EchoModelOps::create_model_info("m", "M", "p", 1024);
    assert!(!m.supports_vision && !m.supports_function_calling && !m.supports_streaming);
}
