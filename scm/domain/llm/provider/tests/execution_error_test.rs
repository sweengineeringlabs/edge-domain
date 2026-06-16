use edge_llm_provider::ExecutionError;
use std::time::Duration;

// Test variant construction
#[test]
fn test_execution_error_auth_failed() {
    let err = ExecutionError::AuthenticationFailed("bad key".into());
    assert!(matches!(err, ExecutionError::AuthenticationFailed(_)));
}

#[test]
fn test_execution_error_rate_limited() {
    let err = ExecutionError::RateLimited {
        retry_after_ms: Some(1000),
    };
    assert!(matches!(err, ExecutionError::RateLimited { .. }));
}

#[test]
fn test_execution_error_context_exceeded() {
    let err = ExecutionError::ContextWindowExceeded {
        max_tokens: 4096,
        requested: 5000,
    };
    assert!(matches!(err, ExecutionError::ContextWindowExceeded { .. }));
}

#[test]
fn test_execution_error_model_not_found() {
    let err = ExecutionError::ModelNotFound("gpt-999".into());
    assert!(matches!(err, ExecutionError::ModelNotFound(_)));
}

#[test]
fn test_execution_error_provider_unavailable() {
    let err = ExecutionError::ProviderUnavailable {
        message: "server down".into(),
    };
    assert!(matches!(err, ExecutionError::ProviderUnavailable { .. }));
}

#[test]
fn test_execution_error_timeout() {
    let err = ExecutionError::Timeout {
        duration_ms: 30000,
    };
    assert!(matches!(err, ExecutionError::Timeout { .. }));
}

#[test]
fn test_execution_error_invalid_request() {
    let err = ExecutionError::InvalidRequest("missing field".into());
    assert!(matches!(err, ExecutionError::InvalidRequest(_)));
}

#[test]
fn test_execution_error_streaming_error() {
    let err = ExecutionError::StreamingError("connection lost".into());
    assert!(matches!(err, ExecutionError::StreamingError(_)));
}

#[test]
fn test_execution_error_cache_error() {
    let err = ExecutionError::CacheError("cache write failed".into());
    assert!(matches!(err, ExecutionError::CacheError(_)));
}

#[test]
fn test_execution_error_tool_call_failed() {
    let err = ExecutionError::ToolCallFailed {
        tool_name: "search".into(),
        reason: "invalid query".into(),
    };
    assert!(matches!(err, ExecutionError::ToolCallFailed { .. }));
}

#[test]
fn test_execution_error_validation_failed() {
    let err = ExecutionError::ValidationFailed("schema mismatch".into());
    assert!(matches!(err, ExecutionError::ValidationFailed(_)));
}

#[test]
fn test_execution_error_content_filtered() {
    let err = ExecutionError::ContentFiltered("policy violation".into());
    assert!(matches!(err, ExecutionError::ContentFiltered(_)));
}

#[test]
fn test_execution_error_quota_exceeded() {
    let err = ExecutionError::QuotaExceeded {
        reset_at_ms: Some(1234567890),
    };
    assert!(matches!(err, ExecutionError::QuotaExceeded { .. }));
}

#[test]
fn test_execution_error_unknown() {
    let err = ExecutionError::Unknown("unknown error".into());
    assert!(matches!(err, ExecutionError::Unknown(_)));
}

// Test is_retryable
#[test]
fn test_execution_error_is_retryable_rate_limited() {
    let err = ExecutionError::RateLimited { retry_after_ms: None };
    assert!(err.is_retryable());
}

#[test]
fn test_execution_error_is_retryable_provider_unavailable() {
    let err = ExecutionError::ProviderUnavailable {
        message: "down".into(),
    };
    assert!(err.is_retryable());
}

#[test]
fn test_execution_error_is_retryable_timeout() {
    let err = ExecutionError::Timeout {
        duration_ms: 5000,
    };
    assert!(err.is_retryable());
}

#[test]
fn test_execution_error_is_retryable_quota_exceeded() {
    let err = ExecutionError::QuotaExceeded { reset_at_ms: None };
    assert!(err.is_retryable());
}

#[test]
fn test_execution_error_not_retryable_auth() {
    let err = ExecutionError::AuthenticationFailed("bad".into());
    assert!(!err.is_retryable());
}

#[test]
fn test_execution_error_not_retryable_model_not_found() {
    let err = ExecutionError::ModelNotFound("missing".into());
    assert!(!err.is_retryable());
}

// Test retry_after
#[test]
fn test_execution_error_retry_after_rate_limited() {
    let err = ExecutionError::RateLimited {
        retry_after_ms: Some(2000),
    };
    assert_eq!(err.retry_after(), Some(Duration::from_millis(2000)));
}

#[test]
fn test_execution_error_retry_after_rate_limited_none() {
    let err = ExecutionError::RateLimited {
        retry_after_ms: None,
    };
    assert_eq!(err.retry_after(), None);
}

#[test]
fn test_execution_error_retry_after_quota() {
    let err = ExecutionError::QuotaExceeded {
        reset_at_ms: Some(3000),
    };
    assert_eq!(err.retry_after(), Some(Duration::from_millis(3000)));
}

#[test]
fn test_execution_error_retry_after_timeout() {
    let err = ExecutionError::Timeout {
        duration_ms: 30000,
    };
    assert_eq!(err.retry_after(), Some(Duration::from_millis(30000)));
}

#[test]
fn test_execution_error_retry_after_auth_none() {
    let err = ExecutionError::AuthenticationFailed("bad".into());
    assert_eq!(err.retry_after(), None);
}

// Test message
#[test]
fn test_execution_error_message_auth() {
    let err = ExecutionError::AuthenticationFailed("invalid key".into());
    assert!(err.message().contains("Authentication failed"));
    assert!(err.message().contains("invalid key"));
}

#[test]
fn test_execution_error_message_rate_limited() {
    let err = ExecutionError::RateLimited {
        retry_after_ms: Some(1000),
    };
    assert!(err.message().contains("Rate limited"));
}

#[test]
fn test_execution_error_message_context_exceeded() {
    let err = ExecutionError::ContextWindowExceeded {
        max_tokens: 4096,
        requested: 5000,
    };
    assert!(err.message().contains("Context window exceeded"));
    assert!(err.message().contains("5000"));
    assert!(err.message().contains("4096"));
}

#[test]
fn test_execution_error_message_tool_call_failed() {
    let err = ExecutionError::ToolCallFailed {
        tool_name: "search".into(),
        reason: "bad query".into(),
    };
    assert!(err.message().contains("Tool call failed"));
    assert!(err.message().contains("search"));
    assert!(err.message().contains("bad query"));
}

// Test serialization
#[test]
fn test_execution_error_serialization_auth() {
    let err = ExecutionError::AuthenticationFailed("key".into());
    let json = serde_json::to_string(&err).expect("serialize");
    let deserialized: ExecutionError = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(
        deserialized,
        ExecutionError::AuthenticationFailed(_)
    ));
}

#[test]
fn test_execution_error_clone() {
    let err1 = ExecutionError::RateLimited {
        retry_after_ms: Some(1000),
    };
    let err2 = err1.clone();
    assert!(matches!(err2, ExecutionError::RateLimited { .. }));
}
