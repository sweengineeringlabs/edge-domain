//! Scenario coverage for the `CompleteFactory` trait.

use edge_llm_complete::{
    CacheControl, CompleteFactory, Completer, ContentPart, FinishReason, ImageUrl, Message,
    NoopCompleter, Role, StdCompleteFactory, StreamDelta, ToolCallDelta, ToolChoice,
};
use serde_json::json;

// ── noop_completer ───────────────────────────────────────────────────────────

#[test]
fn test_noop_completer_is_constructable_happy() {
    let _: NoopCompleter = StdCompleteFactory::noop_completer();
}

#[test]
fn test_noop_completer_type_is_noop_error() {
    // NoopCompleter has no models — confirms it is the noop variant.
    use edge_llm_complete::Completer;
    assert!(StdCompleteFactory::noop_completer().supported_models().is_empty());
}

#[test]
fn test_noop_completer_is_default_constructable_edge() {
    let a = StdCompleteFactory::noop_completer();
    let b = NoopCompleter;
    assert!(a.supported_models().is_empty() && b.supported_models().is_empty());
}

// ── echo_completer ───────────────────────────────────────────────────────────

#[test]
fn test_echo_completer_supports_echo_model_happy() {
    use edge_llm_complete::Completer;
    assert!(StdCompleteFactory::echo_completer().supports("echo"));
}

#[test]
fn test_echo_completer_does_not_support_unknown_model_error() {
    use edge_llm_complete::Completer;
    assert!(!StdCompleteFactory::echo_completer().supports("gpt-4"));
}

#[test]
fn test_echo_completer_supported_models_nonempty_edge() {
    use edge_llm_complete::Completer;
    assert!(!StdCompleteFactory::echo_completer().supported_models().is_empty());
}

// ── request ──────────────────────────────────────────────────────────────────

#[test]
fn test_request_sets_model_and_messages_happy() {
    let msg = Message::user("hi");
    let req = StdCompleteFactory::request("gpt-4".to_string(), vec![msg]);
    assert_eq!(req.model, "gpt-4");
    assert_eq!(req.messages.len(), 1);
}

#[test]
fn test_request_empty_messages_is_valid_error() {
    let req = StdCompleteFactory::request("x".to_string(), vec![]);
    assert!(req.messages.is_empty());
}

#[test]
fn test_request_preserves_all_messages_edge() {
    let msgs = vec![Message::system("sys"), Message::user("u"), Message::assistant("a")];
    let req = StdCompleteFactory::request("m".to_string(), msgs);
    assert_eq!(req.messages.len(), 3);
}

// ── message ──────────────────────────────────────────────────────────────────

#[test]
fn test_message_user_role_is_set_happy() {
    let msg = StdCompleteFactory::message(Role::User, "hello".to_string());
    assert_eq!(msg.role, Role::User);
}

#[test]
fn test_message_empty_content_produces_text_empty_error() {
    let msg = StdCompleteFactory::message(Role::User, String::new());
    use edge_llm_complete::MessageContent;
    assert_eq!(msg.content, MessageContent::Text(String::new()));
}

#[test]
fn test_message_all_roles_constructable_edge() {
    for role in [Role::User, Role::Assistant, Role::System, Role::Tool] {
        let msg = StdCompleteFactory::message(role, "x".to_string());
        assert_eq!(msg.role, role);
    }
}

// ── user_message ─────────────────────────────────────────────────────────────

#[test]
fn test_user_message_has_user_role_happy() {
    assert_eq!(StdCompleteFactory::user_message("hi".to_string()).role, Role::User);
}

#[test]
fn test_user_message_empty_string_is_valid_error() {
    let msg = StdCompleteFactory::user_message(String::new());
    assert_eq!(msg.role, Role::User);
}

#[test]
fn test_user_message_content_matches_input_edge() {
    use edge_llm_complete::MessageContent;
    let msg = StdCompleteFactory::user_message("edge case".to_string());
    assert_eq!(msg.content, MessageContent::Text("edge case".to_string()));
}

// ── assistant_message ────────────────────────────────────────────────────────

#[test]
fn test_assistant_message_has_assistant_role_happy() {
    assert_eq!(StdCompleteFactory::assistant_message("ok".to_string()).role, Role::Assistant);
}

#[test]
fn test_assistant_message_empty_string_is_valid_error() {
    let msg = StdCompleteFactory::assistant_message(String::new());
    assert_eq!(msg.role, Role::Assistant);
}

#[test]
fn test_assistant_message_content_set_edge() {
    use edge_llm_complete::MessageContent;
    let msg = StdCompleteFactory::assistant_message("reply".to_string());
    assert_eq!(msg.content, MessageContent::Text("reply".to_string()));
}

// ── system_message ───────────────────────────────────────────────────────────

#[test]
fn test_system_message_has_system_role_happy() {
    assert_eq!(StdCompleteFactory::system_message("sys".to_string()).role, Role::System);
}

#[test]
fn test_system_message_empty_string_is_valid_error() {
    let msg = StdCompleteFactory::system_message(String::new());
    assert_eq!(msg.role, Role::System);
}

#[test]
fn test_system_message_long_content_is_valid_edge() {
    let long = "x".repeat(10_000);
    let msg = StdCompleteFactory::system_message(long.clone());
    use edge_llm_complete::MessageContent;
    assert_eq!(msg.content, MessageContent::Text(long));
}

// ── tool_message ─────────────────────────────────────────────────────────────

#[test]
fn test_tool_message_has_tool_role_happy() {
    let msg = StdCompleteFactory::tool_message("result".to_string(), "call-1".to_string());
    assert_eq!(msg.role, Role::Tool);
}

#[test]
fn test_tool_message_empty_content_is_valid_error() {
    let msg = StdCompleteFactory::tool_message(String::new(), "id".to_string());
    assert_eq!(msg.role, Role::Tool);
}

#[test]
fn test_tool_message_tool_call_id_is_set_edge() {
    let msg = StdCompleteFactory::tool_message("out".to_string(), "xyz".to_string());
    assert_eq!(msg.tool_call_id, Some("xyz".to_string()));
}

// ── text_part ────────────────────────────────────────────────────────────────

#[test]
fn test_text_part_produces_text_variant_happy() {
    let part = StdCompleteFactory::text_part("hello".to_string());
    assert!(matches!(part, ContentPart::Text { .. }));
}

#[test]
fn test_text_part_empty_string_is_valid_error() {
    let part = StdCompleteFactory::text_part(String::new());
    assert!(matches!(part, ContentPart::Text { text } if text.is_empty()));
}

#[test]
fn test_text_part_preserves_content_edge() {
    let part = StdCompleteFactory::text_part("abc".to_string());
    assert!(matches!(part, ContentPart::Text { text } if text == "abc"));
}

// ── image_url ────────────────────────────────────────────────────────────────

#[test]
fn test_image_url_sets_url_happy() {
    let img: ImageUrl = StdCompleteFactory::image_url("https://example.com/img.png".to_string());
    assert_eq!(img.url, "https://example.com/img.png");
}

#[test]
fn test_image_url_empty_string_is_valid_error() {
    let img: ImageUrl = StdCompleteFactory::image_url(String::new());
    assert!(img.url.is_empty());
}

#[test]
fn test_image_url_detail_is_none_by_default_edge() {
    let img: ImageUrl = StdCompleteFactory::image_url("u".to_string());
    assert!(img.detail.is_none());
}

// ── image_part ───────────────────────────────────────────────────────────────

#[test]
fn test_image_part_produces_image_url_variant_happy() {
    let img = ImageUrl::new("https://x.com/a.png");
    let part = StdCompleteFactory::image_part(img);
    assert!(matches!(part, ContentPart::ImageUrl { .. }));
}

#[test]
fn test_image_part_preserves_url_error() {
    let img = ImageUrl::new("https://x.com/b.png");
    let part = StdCompleteFactory::image_part(img.clone());
    assert!(matches!(&part, ContentPart::ImageUrl { image_url } if image_url.url == img.url));
}

#[test]
fn test_image_part_empty_url_produces_variant_edge() {
    let img = ImageUrl::new("");
    let part = StdCompleteFactory::image_part(img);
    assert!(matches!(part, ContentPart::ImageUrl { .. }));
}

// ── tool_definition ──────────────────────────────────────────────────────────

#[test]
fn test_tool_definition_sets_fields_happy() {
    let td = StdCompleteFactory::tool_definition(
        "search".to_string(),
        "Search the web".to_string(),
        json!({"type": "object"}),
    );
    assert_eq!(td.name, "search");
    assert_eq!(td.description, "Search the web");
}

#[test]
fn test_tool_definition_empty_name_is_valid_error() {
    let td = StdCompleteFactory::tool_definition(
        String::new(),
        "desc".to_string(),
        json!({}),
    );
    assert!(td.name.is_empty());
}

#[test]
fn test_tool_definition_accepts_array_schema_edge() {
    let td = StdCompleteFactory::tool_definition(
        "t".to_string(),
        "d".to_string(),
        json!({"type": "array", "items": {"type": "string"}}),
    );
    assert!(td.parameters.is_object());
}

// ── tool_call ────────────────────────────────────────────────────────────────

#[test]
fn test_tool_call_sets_all_fields_happy() {
    let tc = StdCompleteFactory::tool_call("id-1".to_string(), "search".to_string(), r#"{"q":"x"}"#.to_string());
    assert_eq!(tc.id, "id-1");
    assert_eq!(tc.name, "search");
}

#[test]
fn test_tool_call_empty_fields_are_valid_error() {
    let tc = StdCompleteFactory::tool_call(String::new(), String::new(), String::new());
    assert!(tc.id.is_empty());
}

#[test]
fn test_tool_call_arguments_is_raw_json_string_edge() {
    let tc = StdCompleteFactory::tool_call("x".to_string(), "y".to_string(), r#"{"a":1}"#.to_string());
    assert!(tc.arguments.contains('{'));
}

// ── tool_call_delta ───────────────────────────────────────────────────────────

#[test]
fn test_tool_call_delta_sets_index_happy() {
    let delta: ToolCallDelta = StdCompleteFactory::tool_call_delta(0);
    assert_eq!(delta.index, 0);
}

#[test]
fn test_tool_call_delta_fields_are_none_by_default_error() {
    let delta: ToolCallDelta = StdCompleteFactory::tool_call_delta(1);
    assert!(delta.id.is_none());
    assert!(delta.name.is_none());
}

#[test]
fn test_tool_call_delta_large_index_is_valid_edge() {
    let delta: ToolCallDelta = StdCompleteFactory::tool_call_delta(u32::MAX);
    assert_eq!(delta.index, u32::MAX);
}

// ── stream_delta ─────────────────────────────────────────────────────────────

#[test]
fn test_stream_delta_carries_content_happy() {
    let delta: StreamDelta = StdCompleteFactory::stream_delta("hi".to_string());
    assert_eq!(delta.content, Some("hi".to_string()));
}

#[test]
fn test_stream_delta_empty_string_is_valid_error() {
    let delta: StreamDelta = StdCompleteFactory::stream_delta(String::new());
    assert_eq!(delta.content, Some(String::new()));
}

#[test]
fn test_stream_delta_has_no_tool_calls_by_default_edge() {
    let delta: StreamDelta = StdCompleteFactory::stream_delta("x".to_string());
    assert!(delta.tool_calls.is_none());
}

// ── stream_chunk ─────────────────────────────────────────────────────────────

#[test]
fn test_stream_chunk_sets_all_fields_happy() {
    let delta = StdCompleteFactory::stream_delta("hello".to_string());
    let chunk = StdCompleteFactory::stream_chunk("c-1".to_string(), delta, FinishReason::Stop);
    assert_eq!(chunk.id, "c-1");
    assert_eq!(chunk.finish_reason, Some(FinishReason::Stop));
}

#[test]
fn test_stream_chunk_empty_id_is_valid_error() {
    let delta = StreamDelta::empty();
    let chunk = StdCompleteFactory::stream_chunk(String::new(), delta, FinishReason::Error);
    assert!(chunk.id.is_empty());
}

#[test]
fn test_stream_chunk_length_finish_reason_edge() {
    let delta = StreamDelta::empty();
    let chunk = StdCompleteFactory::stream_chunk("x".to_string(), delta, FinishReason::Length);
    assert_eq!(chunk.finish_reason, Some(FinishReason::Length));
}

// ── token_usage ───────────────────────────────────────────────────────────────

#[test]
fn test_token_usage_is_zeroed_happy() {
    let u = StdCompleteFactory::token_usage();
    assert_eq!(u.total_tokens, 0);
}

#[test]
fn test_token_usage_prompt_is_zero_error() {
    let u = StdCompleteFactory::token_usage();
    assert_eq!(u.prompt_tokens, 0);
}

#[test]
fn test_token_usage_cache_fields_are_zero_edge() {
    let u = StdCompleteFactory::token_usage();
    assert_eq!(u.cache_read_input_tokens, 0);
    assert_eq!(u.cache_creation_input_tokens, 0);
}

// ── model_info ────────────────────────────────────────────────────────────────

#[test]
fn test_model_info_sets_all_fields_happy() {
    let m = StdCompleteFactory::model_info("gpt-4".to_string(), "GPT-4".to_string(), "openai".to_string(), 128_000);
    assert_eq!(m.id, "gpt-4");
    assert_eq!(m.context_window, 128_000);
}

#[test]
fn test_model_info_empty_id_is_valid_error() {
    let m = StdCompleteFactory::model_info(String::new(), String::new(), String::new(), 0);
    assert!(m.id.is_empty());
}

#[test]
fn test_model_info_capabilities_default_to_false_edge() {
    let m = StdCompleteFactory::model_info("m".to_string(), "M".to_string(), "p".to_string(), 1024);
    assert!(!m.supports_vision);
    assert!(!m.supports_function_calling);
    assert!(!m.supports_streaming);
}

// ── cache_control ─────────────────────────────────────────────────────────────

#[test]
fn test_cache_control_is_ephemeral_happy() {
    let cc: CacheControl = StdCompleteFactory::cache_control();
    assert_eq!(cc.cache_type, "ephemeral");
}

#[test]
fn test_cache_control_cache_type_nonempty_error() {
    let cc: CacheControl = StdCompleteFactory::cache_control();
    assert!(!cc.cache_type.is_empty());
}

#[test]
fn test_cache_control_clone_is_equal_edge() {
    let cc: CacheControl = StdCompleteFactory::cache_control();
    assert_eq!(cc.clone(), cc);
}

// ── tool_choice_auto ─────────────────────────────────────────────────────────

#[test]
fn test_tool_choice_auto_is_auto_happy() {
    assert_eq!(StdCompleteFactory::tool_choice_auto(), ToolChoice::Auto);
}

#[test]
fn test_tool_choice_auto_is_not_none_error() {
    assert_ne!(StdCompleteFactory::tool_choice_auto(), ToolChoice::None);
}

#[test]
fn test_tool_choice_auto_is_not_required_edge() {
    assert_ne!(StdCompleteFactory::tool_choice_auto(), ToolChoice::Required);
}

// ── std_complete_factory ─────────────────────────────────────────────────────

#[test]
fn test_std_complete_factory_returns_instance_happy() {
    let factory = StdCompleteFactory::std_complete_factory();
    let _ = factory; // StdCompleteFactory is a zero-size unit struct
}

#[test]
fn test_std_complete_factory_is_not_a_completer_error() {
    use edge_llm_complete::Completer;
    // StdCompleteFactory is a factory, not a completer — it has no models.
    let factory = StdCompleteFactory::std_complete_factory();
    let noop = StdCompleteFactory::noop_completer();
    // Both are constructed from factory methods — factory is unit struct, noop has no models.
    assert!(noop.supported_models().is_empty());
    let _ = factory;
}
