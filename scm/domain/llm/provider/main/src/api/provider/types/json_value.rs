use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Local JSON-value AST — replaces `serde_json::Value` in api/ type positions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JsonValue {
    /// JSON `null`.
    Null,
    /// JSON boolean.
    Bool(bool),
    /// JSON number, represented as `f64`.
    Number(f64),
    /// JSON string.
    String(String),
    /// JSON array.
    Array(Vec<JsonValue>),
    /// JSON object.
    Object(BTreeMap<String, JsonValue>),
}
