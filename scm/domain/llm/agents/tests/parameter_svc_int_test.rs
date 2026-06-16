//! Integration tests for Parameter type re-export via parameter_svc.rs.

use edge_llm_agent::Parameter;

/// @covers: Parameter type re-export
#[test]
fn test_svc_parameter_happy_type_can_be_constructed() {
    let param = Parameter {
        name: "input".to_string(),
        description: "Test input".to_string(),
        param_type: "string".to_string(),
        required: true,
    };
    assert_eq!(param.name, "input");
}

/// @covers: Parameter type re-export — all fields
#[test]
fn test_svc_parameter_happy_all_fields_are_accessible() {
    let param = Parameter {
        name: "data".to_string(),
        description: "Input data".to_string(),
        param_type: "object".to_string(),
        required: false,
    };
    assert_eq!(param.name, "data");
    assert_eq!(param.description, "Input data");
    assert_eq!(param.param_type, "object");
    assert!(!param.required);
}

/// @covers: Parameter type re-export — name field
#[test]
fn test_svc_parameter_happy_name_field_stores_value() {
    let param = Parameter {
        name: "config".to_string(),
        description: "Configuration".to_string(),
        param_type: "string".to_string(),
        required: true,
    };
    assert_eq!(param.name, "config");
}

/// @covers: Parameter type re-export — description field
#[test]
fn test_svc_parameter_happy_description_field_stores_value() {
    let param = Parameter {
        name: "timeout".to_string(),
        description: "Request timeout in seconds".to_string(),
        param_type: "number".to_string(),
        required: true,
    };
    assert_eq!(param.description, "Request timeout in seconds");
}

/// @covers: Parameter type re-export — param_type field
#[test]
fn test_svc_parameter_happy_param_type_field_stores_value() {
    let param = Parameter {
        name: "count".to_string(),
        description: "Item count".to_string(),
        param_type: "number".to_string(),
        required: false,
    };
    assert_eq!(param.param_type, "number");
}

/// @covers: Parameter type re-export — required field
#[test]
fn test_svc_parameter_happy_required_field_true() {
    let param = Parameter {
        name: "api_key".to_string(),
        description: "API key".to_string(),
        param_type: "string".to_string(),
        required: true,
    };
    assert!(param.required);
}

/// @covers: Parameter type re-export — required field
#[test]
fn test_svc_parameter_happy_required_field_false() {
    let param = Parameter {
        name: "optional_param".to_string(),
        description: "Optional parameter".to_string(),
        param_type: "string".to_string(),
        required: false,
    };
    assert!(!param.required);
}

/// @covers: Parameter type re-export — Clone trait
#[test]
fn test_svc_parameter_happy_can_be_cloned() {
    let param = Parameter {
        name: "original".to_string(),
        description: "Original parameter".to_string(),
        param_type: "string".to_string(),
        required: true,
    };
    let cloned = param.clone();
    assert_eq!(cloned.name, "original");
    assert_eq!(cloned.description, "Original parameter");
}

/// @covers: Parameter type re-export — Debug trait
#[test]
fn test_svc_parameter_happy_debug_format_includes_fields() {
    let param = Parameter {
        name: "test".to_string(),
        description: "Test parameter".to_string(),
        param_type: "string".to_string(),
        required: true,
    };
    let debug_str = format!("{:?}", param);
    assert!(debug_str.contains("test"));
}

/// @covers: Parameter type re-export — edge case empty strings
#[test]
fn test_svc_parameter_edge_empty_string_values() {
    let param = Parameter {
        name: String::new(),
        description: String::new(),
        param_type: String::new(),
        required: false,
    };
    assert_eq!(param.name, "");
    assert_eq!(param.description, "");
    assert_eq!(param.param_type, "");
}

/// @covers: Parameter type re-export — various type values
#[test]
fn test_svc_parameter_happy_various_param_types() {
    let types = vec!["string", "number", "object", "array", "boolean"];
    for type_name in types {
        let param = Parameter {
            name: "test".to_string(),
            description: "Test".to_string(),
            param_type: type_name.to_string(),
            required: true,
        };
        assert_eq!(param.param_type, type_name);
    }
}
