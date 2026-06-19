use edge_llm_complete::ModelInfo;

#[test]
fn test_new_sets_id_and_provider() {
    let m = ModelInfo::new("gpt-4", "GPT-4", "openai", 128_000);
    assert_eq!(m.id, "gpt-4");
    assert_eq!(m.provider, "openai");
    assert_eq!(m.context_window, 128_000);
}

#[test]
fn test_capabilities_default_to_false() {
    let m = ModelInfo::new("m", "M", "p", 0);
    assert!(!m.supports_vision && !m.supports_function_calling && !m.supports_streaming);
}

#[test]
fn test_roundtrip_serialization() {
    let m = ModelInfo::new("gpt-4o", "GPT-4o", "openai", 128_000);
    let json = serde_json::to_string(&m).unwrap();
    let back: ModelInfo = serde_json::from_str(&json).unwrap();
    assert_eq!(m, back);
}
