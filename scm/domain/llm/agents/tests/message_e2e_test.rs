#![allow(clippy::unwrap_used, clippy::expect_used)]
use edge_llm_agent::{CacheControl, Message, Role, ToolCall};

/// @covers: user
#[test]
fn test_message_user_constructor() {
    let msg = Message::user("hello");
    assert_eq!(msg.role, Role::User);
}

#[test]
fn test_message_user_with_string() {
    let msg = Message::user("test message".to_string());
    assert_eq!(msg.role, Role::User);
}

/// @covers: assistant
#[test]
fn test_message_assistant_constructor() {
    let msg = Message::assistant("response");
    assert_eq!(msg.role, Role::Assistant);
}

/// @covers: system
#[test]
fn test_message_system_constructor() {
    let msg = Message::system("system prompt");
    assert_eq!(msg.role, Role::System);
}

/// @covers: tool
#[test]
fn test_message_tool_constructor() {
    let msg = Message::tool("tool result", "call-id-123");
    assert_eq!(msg.role, Role::Tool);
    assert_eq!(msg.tool_call_id.as_deref(), Some("call-id-123"));
}

#[test]
fn test_message_user_defaults() {
    let msg = Message::user("test");
    assert_eq!(msg.role, Role::User);
    assert_eq!(msg.name, None);
    assert_eq!(msg.tool_call_id, None);
    assert!(msg.tool_calls.is_empty());
    assert_eq!(msg.cache_control, None);
}

#[test]
fn test_message_tool_with_string_content() {
    let msg = Message::tool("result".to_string(), "call-456");
    assert_eq!(msg.role, Role::Tool);
    assert_eq!(msg.tool_call_id.as_deref(), Some("call-456"));
}

#[test]
fn test_message_equality() {
    let msg1 = Message::user("hello");
    let msg2 = Message::user("hello");
    assert_eq!(msg1, msg2);
}

#[test]
fn test_message_inequality_different_role() {
    let msg1 = Message::user("hello");
    let msg2 = Message::assistant("hello");
    assert_ne!(msg1, msg2);
}

#[test]
fn test_message_inequality_different_content() {
    let msg1 = Message::user("hello");
    let msg2 = Message::user("goodbye");
    assert_ne!(msg1, msg2);
}

#[test]
fn test_message_clone() {
    let msg1 = Message::user("test");
    let msg2 = msg1.clone();
    assert_eq!(msg1, msg2);
}

#[test]
fn test_message_debug_format() {
    let msg = Message::user("test");
    let debug_str = format!("{:?}", msg);
    assert!(debug_str.contains("User"));
}

#[test]
fn test_message_with_tool_calls() {
    let mut msg = Message::assistant("response");
    let tool_call = ToolCall {
        id: "call-1".to_string(),
        name: "search".to_string(),
        arguments: r#"{"query": "test"}"#.to_string(),
    };
    msg.tool_calls.push(tool_call);
    assert_eq!(msg.tool_calls.len(), 1);
    assert_eq!(msg.tool_calls[0].name, "search");
}

#[test]
fn test_message_with_cache_control() {
    let mut msg = Message::user("test");
    msg.cache_control = Some(CacheControl {
        cache_type: "ephemeral".to_string(),
    });
    assert!(msg.cache_control.unwrap());
    assert_eq!(msg.cache_control.as_ref().unwrap().cache_type, "ephemeral");
}

#[test]
fn test_message_with_name() {
    let mut msg = Message::user("test");
    msg.name = Some("alice".to_string());
    assert_eq!(msg.name.as_deref(), Some("alice"));
}

#[test]
fn test_message_serialization_user() {
    let msg = Message::user("hello");
    let json = serde_json::to_string(&msg).expect("serialize");
    let deserialized: Message = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(msg, deserialized);
}

#[test]
fn test_message_serialization_assistant() {
    let msg = Message::assistant("response");
    let json = serde_json::to_string(&msg).expect("serialize");
    let deserialized: Message = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(msg, deserialized);
}

#[test]
fn test_message_serialization_system() {
    let msg = Message::system("instructions");
    let json = serde_json::to_string(&msg).expect("serialize");
    let deserialized: Message = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(msg, deserialized);
}

#[test]
fn test_message_serialization_tool() {
    let msg = Message::tool("result", "call-123");
    let json = serde_json::to_string(&msg).expect("serialize");
    let deserialized: Message = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(msg, deserialized);
}

#[test]
fn test_message_serialization_with_tool_calls() {
    let mut msg = Message::assistant("response");
    let tool_call = ToolCall {
        id: "call-1".to_string(),
        name: "execute".to_string(),
        arguments: "{}".to_string(),
    };
    msg.tool_calls.push(tool_call);
    let json = serde_json::to_string(&msg).expect("serialize");
    let deserialized: Message = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(msg, deserialized);
}

#[test]
fn test_message_serialization_with_cache_control() {
    let mut msg = Message::user("test");
    msg.cache_control = Some(CacheControl {
        cache_type: "persistent".to_string(),
    });
    let json = serde_json::to_string(&msg).expect("serialize");
    let deserialized: Message = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(msg, deserialized);
}

#[test]
fn test_message_json_structure_user() {
    let msg = Message::user("hello");
    let json = serde_json::to_value(&msg).expect("to_value");
    assert_eq!(json["role"], "User");
    assert!(json["name"].is_null());
    assert!(json["tool_call_id"].is_null());
}

#[test]
fn test_message_json_structure_tool() {
    let msg = Message::tool("result", "call-123");
    let json = serde_json::to_value(&msg).expect("to_value");
    assert_eq!(json["role"], "Tool");
    assert_eq!(json["tool_call_id"], "call-123");
}

#[test]
fn test_tool_call_construction() {
    let tc = ToolCall {
        id: "123".to_string(),
        name: "my_tool".to_string(),
        arguments: "{}".to_string(),
    };
    assert_eq!(tc.id, "123");
    assert_eq!(tc.name, "my_tool");
    assert_eq!(tc.arguments, "{}");
}

#[test]
fn test_tool_call_clone() {
    let tc1 = ToolCall {
        id: "123".to_string(),
        name: "tool".to_string(),
        arguments: "{}".to_string(),
    };
    let tc2 = tc1.clone();
    assert_eq!(tc1, tc2);
}

#[test]
fn test_tool_call_equality() {
    let tc1 = ToolCall {
        id: "123".to_string(),
        name: "tool".to_string(),
        arguments: "{}".to_string(),
    };
    let tc2 = ToolCall {
        id: "123".to_string(),
        name: "tool".to_string(),
        arguments: "{}".to_string(),
    };
    assert_eq!(tc1, tc2);
}

#[test]
fn test_tool_call_inequality() {
    let tc1 = ToolCall {
        id: "123".to_string(),
        name: "tool1".to_string(),
        arguments: "{}".to_string(),
    };
    let tc2 = ToolCall {
        id: "123".to_string(),
        name: "tool2".to_string(),
        arguments: "{}".to_string(),
    };
    assert_ne!(tc1, tc2);
}

#[test]
fn test_tool_call_serialization() {
    let tc = ToolCall {
        id: "call-1".to_string(),
        name: "search".to_string(),
        arguments: r#"{"query": "test"}"#.to_string(),
    };
    let json = serde_json::to_string(&tc).expect("serialize");
    let deserialized: ToolCall = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(tc, deserialized);
}

#[test]
fn test_message_multiple_tool_calls() {
    let mut msg = Message::assistant("checking tools");
    msg.tool_calls.push(ToolCall {
        id: "1".to_string(),
        name: "tool1".to_string(),
        arguments: "{}".to_string(),
    });
    msg.tool_calls.push(ToolCall {
        id: "2".to_string(),
        name: "tool2".to_string(),
        arguments: "{}".to_string(),
    });
    assert_eq!(msg.tool_calls.len(), 2);
    assert_eq!(msg.tool_calls[0].id, "1");
    assert_eq!(msg.tool_calls[1].id, "2");
}
