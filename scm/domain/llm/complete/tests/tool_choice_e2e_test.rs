//! Scenario coverage for `ToolChoice`.

use edge_llm_complete::ToolChoice;

#[test]
fn test_tool_choice_auto_is_distinct_from_none_happy() {
    assert_ne!(ToolChoice::Auto, ToolChoice::None);
}

#[test]
fn test_tool_choice_required_is_distinct_from_auto_error() {
    assert_ne!(ToolChoice::Required, ToolChoice::Auto);
}

#[test]
fn test_tool_choice_all_variants_clone_and_eq_edge() {
    for choice in [ToolChoice::Auto, ToolChoice::None, ToolChoice::Required] {
        assert_eq!(choice.clone(), choice);
    }
}
