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
            serde_json::Value::Array(items) => {
                JsonValue::Array(items.into_iter().map(JsonValue::from).collect())
            }
            serde_json::Value::Object(map) => JsonValue::Object(
                map.into_iter()
                    .map(|(k, v)| (k, JsonValue::from(v)))
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
            JsonValue::Array(items) => {
                serde_json::Value::Array(items.into_iter().map(serde_json::Value::from).collect())
            }
            JsonValue::Object(map) => serde_json::Value::Object(
                map.into_iter()
                    .map(|(k, v)| (k, serde_json::Value::from(v)))
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_serde_json_value_round_trips_object() {
        // Numbers round-trip through f64, so use a fixture whose numeric fields
        // are already f64-representable (an integer literal like `1` would
        // compare unequal to the round-tripped `1.0`).
        let original = serde_json::json!({"a": 1.0, "b": [true, null, "x"]});
        let local: JsonValue = original.clone().into();
        let back: serde_json::Value = local.into();
        assert_eq!(back, original);
    }

    #[test]
    fn test_from_json_value_number_converts_to_f64() {
        let local = JsonValue::Number(42.0);
        let value: serde_json::Value = local.into();
        assert_eq!(value, serde_json::json!(42.0));
    }
}
