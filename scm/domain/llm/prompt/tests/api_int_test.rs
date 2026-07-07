//! Layer-level coverage for small api/ value types (SEA `sea_layer_test_coverage`
//! Option C): constructs and asserts every Request/Response/marker type that has
//! no dedicated test file of its own.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    CacheBuildRequest, CacheBuildResponse, ClearVariablesRequest, CompletenessRequest,
    CompletenessResponse, ContextBuildRequest, ContextBuildResponse, CountTokensRequest,
    CountTokensResponse, EstimateTokensRequest, EstimateTokensResponse, ExactnessRequest,
    ExactnessResponse, JsonValue, ListByCategoryRequest, ListByCategoryResponse,
    ListTemplatesRequest, ListTemplatesResponse, PromptMetadataRequest, PromptMetadataResponse,
    PromptVariableKindRequest, PromptVariableKindResponse, RegisterVariableRequest, RenderContext,
    RenderRequest, RenderResponse, TemplateLookupRequest, TemplateLookupResponse,
    TemplateValidationRequest, TokenizerNameRequest, TokenizerNameResponse, Variable, VariableKind,
    VariableLookupRequest, VariableLookupResponse,
};

#[test]
fn test_cache_build_request_holds_context_and_rendered() {
    let ctx = RenderContext::new();
    let req = CacheBuildRequest {
        context: &ctx,
        rendered: "x".to_string(),
    };
    assert_eq!(req.rendered, "x");
}

#[test]
fn test_cache_build_response_holds_flattened_fields() {
    let resp = CacheBuildResponse {
        key: "k".into(),
        rendered: "r".into(),
        token_count: 1,
        created_at: 0,
        ttl_seconds: 3600,
        hit_count: 0,
    };
    assert_eq!(resp.key, "k");
}

#[test]
fn test_clear_variables_request_is_unit() {
    assert_eq!(std::mem::size_of_val(&ClearVariablesRequest), 0);
}

#[test]
fn test_completeness_request_is_unit() {
    assert_eq!(std::mem::size_of_val(&CompletenessRequest), 0);
}

#[test]
fn test_completeness_response_holds_flag() {
    assert!(CompletenessResponse { complete: true }.complete);
}

#[test]
fn test_context_build_request_is_unit() {
    assert_eq!(std::mem::size_of_val(&ContextBuildRequest), 0);
}

#[test]
fn test_context_build_response_holds_flattened_fields() {
    let resp = ContextBuildResponse {
        variables: Default::default(),
        metadata: Default::default(),
        template_id: None,
    };
    assert!(resp.variables.is_empty());
}

#[test]
fn test_count_tokens_request_holds_text() {
    assert_eq!(CountTokensRequest { text: "x" }.text, "x");
}

#[test]
fn test_count_tokens_response_holds_count() {
    assert_eq!(CountTokensResponse { count: 3 }.count, 3);
}

#[test]
fn test_estimate_tokens_request_holds_text() {
    assert_eq!(EstimateTokensRequest { text: "x" }.text, "x");
}

#[test]
fn test_estimate_tokens_response_holds_count() {
    assert_eq!(EstimateTokensResponse { count: 3 }.count, 3);
}

#[test]
fn test_exactness_request_is_unit() {
    assert_eq!(std::mem::size_of_val(&ExactnessRequest), 0);
}

#[test]
fn test_exactness_response_holds_flag() {
    assert!(!ExactnessResponse { exact: false }.exact);
}

#[test]
fn test_json_value_variants_construct() {
    let from_number: JsonValue = serde_json::json!(1).into();
    assert_eq!(from_number, JsonValue::Number(1.0));
    assert_ne!(JsonValue::Bool(true), JsonValue::Bool(false));
}

#[test]
fn test_list_by_category_request_holds_category() {
    assert_eq!(ListByCategoryRequest { category: "code" }.category, "code");
}

#[test]
fn test_list_by_category_response_holds_templates() {
    let resp = ListByCategoryResponse { templates: vec![] };
    assert!(resp.templates.is_empty());
}

#[test]
fn test_list_templates_request_is_unit() {
    assert_eq!(std::mem::size_of_val(&ListTemplatesRequest), 0);
}

#[test]
fn test_list_templates_response_holds_templates() {
    let resp = ListTemplatesResponse { templates: vec![] };
    assert!(resp.templates.is_empty());
}

#[test]
fn test_prompt_metadata_request_is_unit() {
    assert_eq!(std::mem::size_of_val(&PromptMetadataRequest), 0);
}

#[test]
fn test_prompt_metadata_response_holds_flattened_fields() {
    let resp = PromptMetadataResponse {
        id: "i".into(),
        name: "n".into(),
        version: "1".into(),
        variables: vec![],
        description: None,
        base_token_count: 0,
        tags: vec![],
    };
    assert_eq!(resp.id, "i");
}

#[test]
fn test_prompt_variable_kind_request_holds_name() {
    assert_eq!(PromptVariableKindRequest { name: "x" }.name, "x");
}

#[test]
fn test_prompt_variable_kind_response_holds_kind() {
    let resp = PromptVariableKindResponse {
        kind: Some(VariableKind::String),
    };
    assert_eq!(resp.kind, Some(VariableKind::String));
}

#[test]
fn test_register_variable_request_holds_name_and_var() {
    let var = Variable::new("a".into(), VariableKind::String);
    let req = RegisterVariableRequest {
        name: "a".to_string(),
        var: &var,
    };
    assert_eq!(req.name, "a");
}

#[test]
fn test_render_request_holds_context() {
    let ctx = RenderContext::new();
    assert_eq!(RenderRequest { context: &ctx }.context.variable_count(), 0);
}

#[test]
fn test_render_response_holds_rendered_text() {
    assert_eq!(
        RenderResponse {
            rendered: "x".into()
        }
        .rendered,
        "x"
    );
}

#[test]
fn test_template_lookup_request_holds_id() {
    assert_eq!(TemplateLookupRequest { id: "a" }.id, "a");
}

#[test]
fn test_template_lookup_response_holds_template() {
    let resp = TemplateLookupResponse { template: None };
    assert!(resp.template.is_none());
}

#[test]
fn test_template_validation_request_is_unit() {
    assert_eq!(std::mem::size_of_val(&TemplateValidationRequest), 0);
}

#[test]
fn test_tokenizer_name_request_is_unit() {
    assert_eq!(std::mem::size_of_val(&TokenizerNameRequest), 0);
}

#[test]
fn test_tokenizer_name_response_holds_name() {
    assert_eq!(TokenizerNameResponse { name: "x" }.name, "x");
}

#[test]
fn test_variable_lookup_request_holds_name() {
    assert_eq!(VariableLookupRequest { name: "a" }.name, "a");
}

#[test]
fn test_variable_lookup_response_holds_variable() {
    let resp = VariableLookupResponse { variable: None };
    assert!(resp.variable.is_none());
}
