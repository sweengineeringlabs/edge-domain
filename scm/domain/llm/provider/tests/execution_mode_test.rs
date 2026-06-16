use edge_llm_provider::ExecutionMode;

#[test]
fn test_execution_mode_async_variant_exists() {
    let _ = ExecutionMode::Async;
}

#[test]
fn test_execution_mode_long_running_variant_exists() {
    let _ = ExecutionMode::LongRunning;
}

#[test]
fn test_execution_mode_streaming_variant_exists() {
    let _ = ExecutionMode::Streaming;
}

#[test]
fn test_execution_mode_is_streaming_happy_streaming() {
    assert!(ExecutionMode::Streaming.is_streaming());
}

#[test]
fn test_execution_mode_is_streaming_edge_async() {
    assert!(!ExecutionMode::Async.is_streaming());
}

#[test]
fn test_execution_mode_is_streaming_edge_long_running() {
    assert!(!ExecutionMode::LongRunning.is_streaming());
}

#[test]
fn test_execution_mode_is_async_happy_async() {
    assert!(ExecutionMode::Async.is_async());
}

#[test]
fn test_execution_mode_is_async_happy_streaming() {
    assert!(ExecutionMode::Streaming.is_async());
}

#[test]
fn test_execution_mode_is_async_edge_long_running() {
    assert!(!ExecutionMode::LongRunning.is_async());
}

#[test]
fn test_execution_mode_equality() {
    assert_eq!(ExecutionMode::Async, ExecutionMode::Async);
    assert_ne!(ExecutionMode::Async, ExecutionMode::Streaming);
}

#[test]
fn test_execution_mode_clone() {
    let mode = ExecutionMode::Streaming;
    let cloned = mode.clone();
    assert_eq!(mode, cloned);
}

#[test]
fn test_execution_mode_copy() {
    let mode = ExecutionMode::LongRunning;
    let copied = mode;
    assert_eq!(mode, copied);
}

#[test]
fn test_execution_mode_debug_format() {
    assert_eq!(format!("{:?}", ExecutionMode::Async), "Async");
    assert_eq!(format!("{:?}", ExecutionMode::LongRunning), "LongRunning");
    assert_eq!(format!("{:?}", ExecutionMode::Streaming), "Streaming");
}

#[test]
fn test_execution_mode_serialization_async() {
    let mode = ExecutionMode::Async;
    let json = serde_json::to_string(&mode).expect("serialize");
    let deserialized: ExecutionMode = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(mode, deserialized);
}

#[test]
fn test_execution_mode_serialization_long_running() {
    let mode = ExecutionMode::LongRunning;
    let json = serde_json::to_string(&mode).expect("serialize");
    let deserialized: ExecutionMode = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(mode, deserialized);
}

#[test]
fn test_execution_mode_serialization_streaming() {
    let mode = ExecutionMode::Streaming;
    let json = serde_json::to_string(&mode).expect("serialize");
    let deserialized: ExecutionMode = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(mode, deserialized);
}

#[test]
fn test_execution_mode_json_format_async() {
    let mode = ExecutionMode::Async;
    let json = serde_json::to_string(&mode).expect("serialize");
    assert_eq!(json, "\"Async\"");
}

#[test]
fn test_execution_mode_json_format_long_running() {
    let mode = ExecutionMode::LongRunning;
    let json = serde_json::to_string(&mode).expect("serialize");
    assert_eq!(json, "\"LongRunning\"");
}

#[test]
fn test_execution_mode_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mode1 = ExecutionMode::Async;
    let mode2 = ExecutionMode::Async;

    let mut hasher1 = DefaultHasher::new();
    mode1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    mode2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2);
}

#[test]
fn test_execution_mode_different_hashes() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mode1 = ExecutionMode::Async;
    let mode2 = ExecutionMode::Streaming;

    let mut hasher1 = DefaultHasher::new();
    mode1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    mode2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_ne!(hash1, hash2);
}
