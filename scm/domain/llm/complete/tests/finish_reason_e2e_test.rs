use edge_llm_complete::FinishReason;

#[test]
fn test_all_variants_are_distinct() {
    assert_ne!(FinishReason::Stop, FinishReason::Length);
    assert_ne!(FinishReason::ToolCalls, FinishReason::ContentFilter);
    assert_ne!(FinishReason::Error, FinishReason::Stop);
}

#[test]
fn test_default_is_stop() {
    assert_eq!(FinishReason::default(), FinishReason::Stop);
}

#[test]
fn test_roundtrip_serialization() {
    for variant in [
        FinishReason::Stop,
        FinishReason::Length,
        FinishReason::ToolCalls,
        FinishReason::ContentFilter,
        FinishReason::Error,
    ] {
        let json = serde_json::to_string(&variant).unwrap();
        let back: FinishReason = serde_json::from_str(&json).unwrap();
        assert_eq!(variant, back);
    }
}
