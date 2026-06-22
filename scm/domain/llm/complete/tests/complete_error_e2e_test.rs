use edge_llm_complete::CompleteError;
use std::time::Duration;

#[test]
fn test_is_retryable_network_error_returns_true() {
    assert!(CompleteError::NetworkError("timeout".to_string()).is_retryable());
}

#[test]
fn test_is_retryable_invalid_request_returns_false() {
    assert!(!CompleteError::InvalidRequest("bad".to_string()).is_retryable());
}

#[test]
fn test_is_retryable_rate_limited_returns_true() {
    assert!(CompleteError::RateLimited {
        retry_after_ms: None
    }
    .is_retryable());
}

#[test]
fn test_retry_after_rate_limited_with_ms_returns_duration() {
    let err = CompleteError::RateLimited {
        retry_after_ms: Some(5_000),
    };
    assert_eq!(err.retry_after(), Some(Duration::from_millis(5_000)));
}

#[test]
fn test_retry_after_network_error_returns_none() {
    assert!(CompleteError::NetworkError("x".to_string())
        .retry_after()
        .is_none());
}

#[test]
fn test_is_retryable_timeout_returns_true() {
    assert!(CompleteError::Timeout(3_000).is_retryable());
}

#[test]
fn test_provider_not_found_display_contains_provider_name() {
    let err = CompleteError::ProviderNotFound("anthropic".to_string());
    assert!(format!("{err}").contains("anthropic"));
}

#[test]
fn test_model_not_found_display_contains_model_name() {
    let err = CompleteError::ModelNotFound("gpt-4".to_string());
    assert!(format!("{err}").contains("gpt-4"));
}

#[test]
fn test_context_length_exceeded_display_contains_counts() {
    let err = CompleteError::ContextLengthExceeded {
        used: 200_000,
        max: 128_000,
    };
    let msg = format!("{err}");
    assert!(msg.contains("200000") && msg.contains("128000"));
}

#[test]
fn test_io_error_converts_from_std_io_error() {
    let io: std::io::Error = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
    let err: CompleteError = io.into();
    assert!(matches!(err, CompleteError::IoError(_)));
}
