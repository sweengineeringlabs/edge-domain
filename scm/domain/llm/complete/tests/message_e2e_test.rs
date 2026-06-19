use edge_llm_complete::{Message, MessageContent, Role};

#[test]
fn test_user_message_has_user_role() {
    assert_eq!(Message::user("hi").role, Role::User);
}

#[test]
fn test_assistant_message_has_assistant_role() {
    assert_eq!(Message::assistant("ok").role, Role::Assistant);
}

#[test]
fn test_system_message_has_system_role() {
    assert_eq!(Message::system("be helpful").role, Role::System);
}

#[test]
fn test_tool_message_has_tool_role_and_call_id() {
    let msg = Message::tool("result", "call-1");
    assert_eq!(msg.role, Role::Tool);
    assert_eq!(msg.tool_call_id, Some("call-1".to_string()));
}

#[test]
fn test_message_content_is_text() {
    let msg = Message::user("hello");
    assert_eq!(msg.content, MessageContent::Text("hello".to_string()));
}

#[test]
fn test_message_default_has_no_tool_calls() {
    assert!(Message::default().tool_calls.is_empty());
}
