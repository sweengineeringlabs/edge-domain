//! Tests for `ToolCallDeltaBuilder` setter methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ToolCallDeltaBuilder;

/// @covers: ToolCallDeltaBuilder::id — sets the call identifier
#[test]
fn test_id() {
    let d = ToolCallDeltaBuilder::new(0).id("call-1".to_string()).build();
    assert_eq!(d.id.as_deref(), Some("call-1"));
}

/// @covers: ToolCallDeltaBuilder::id — absent by default
#[test]
fn test_id_absent_by_default_edge() {
    let d = ToolCallDeltaBuilder::new(0).build();
    assert!(d.id.is_none());
}

/// @covers: ToolCallDeltaBuilder::name — sets the function name
#[test]
fn test_name() {
    let d = ToolCallDeltaBuilder::new(0).name("search".to_string()).build();
    assert_eq!(d.name.as_deref(), Some("search"));
}

/// @covers: ToolCallDeltaBuilder::arguments — sets the arguments string
#[test]
fn test_arguments() {
    let d = ToolCallDeltaBuilder::new(0).arguments("{\"q\":\"test\"}".to_string()).build();
    assert!(d.arguments.is_some());
}

/// @covers: ToolCallDeltaBuilder::build — index is set from constructor
#[test]
fn test_build() {
    let d = ToolCallDeltaBuilder::new(5).build();
    assert_eq!(d.index, 5);
}

/// @covers: ToolCallDeltaBuilder::build — zero index with empty fields
#[test]
fn test_build_index_zero_edge() {
    let d = ToolCallDeltaBuilder::new(0).build();
    assert_eq!(d.index, 0);
    assert!(d.name.is_none());
    assert!(d.arguments.is_none());
}
