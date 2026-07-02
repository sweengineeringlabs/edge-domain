//! `JsonValue` — local JSON value representation so api/ never names the
//! foreign `serde_json::Value` type directly (SEA `no_foreign_type`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A JSON-compatible value, independent of any external JSON crate.
///
/// Conversions to/from `serde_json::Value` live in `core/`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JsonValue {
    /// JSON `null`.
    Null,
    /// JSON boolean.
    Bool(bool),
    /// JSON number, stored as a 64-bit float.
    Number(f64),
    /// JSON string.
    String(String),
    /// JSON array.
    Array(Vec<JsonValue>),
    /// JSON object, keyed by string with deterministic iteration order.
    Object(BTreeMap<String, JsonValue>),
}
