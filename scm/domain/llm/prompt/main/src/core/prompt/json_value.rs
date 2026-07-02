//! Conversions between [`JsonValue`] and `serde_json::Value`.

use std::collections::BTreeMap;

use crate::api::JsonValue;

impl From<serde_json::Value> for JsonValue {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => JsonValue::Null,
            serde_json::Value::Bool(b) => JsonValue::Bool(b),
            serde_json::Value::Number(n) => JsonValue::Number(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::String(s) => JsonValue::String(s),
            serde_json::Value::Array(a) => {
                JsonValue::Array(a.into_iter().map(Into::into).collect())
            }
            serde_json::Value::Object(o) => JsonValue::Object(
                o.into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect::<BTreeMap<_, _>>(),
            ),
        }
    }
}

impl From<JsonValue> for serde_json::Value {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::Null => serde_json::Value::Null,
            JsonValue::Bool(b) => serde_json::Value::Bool(b),
            JsonValue::Number(n) => serde_json::Number::from_f64(n)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            JsonValue::String(s) => serde_json::Value::String(s),
            JsonValue::Array(a) => {
                serde_json::Value::Array(a.into_iter().map(Into::into).collect())
            }
            JsonValue::Object(o) => {
                serde_json::Value::Object(o.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
        }
    }
}

impl JsonValue {
    /// Render this value as a display string, mirroring how
    /// `StaticPrompt::substitute` needs to show a value in a template.
    pub(crate) fn display(&self) -> String {
        match self {
            JsonValue::String(s) => s.clone(),
            other => serde_json::Value::from(other.clone()).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_serde_json_value_round_trips_string() {
        let v: JsonValue = serde_json::json!("Ada").into();
        assert_eq!(v, JsonValue::String("Ada".to_string()));
    }

    #[test]
    fn test_from_serde_json_value_round_trips_number() {
        let v: JsonValue = serde_json::json!(42).into();
        assert_eq!(v, JsonValue::Number(42.0));
    }

    #[test]
    fn test_into_serde_json_value_round_trips_object() {
        let mut map = BTreeMap::new();
        map.insert("a".to_string(), JsonValue::Bool(true));
        let v = JsonValue::Object(map);
        let round_tripped: serde_json::Value = v.into();
        assert_eq!(round_tripped, serde_json::json!({"a": true}));
    }

    #[test]
    fn test_display_string_variant_is_unquoted() {
        assert_eq!(JsonValue::String("hi".to_string()).display(), "hi");
    }

    #[test]
    fn test_display_number_variant_is_json_text() {
        assert_eq!(JsonValue::Number(3.0).display(), "3.0");
    }
}
