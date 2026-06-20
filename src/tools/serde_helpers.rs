//! Serde helpers for lenient MCP argument parsing.
//!
//! Some MCP clients serialize complex values (arrays, objects) as JSON strings
//! instead of native JSON structures. These helpers accept both forms.

use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use serde_json::Value;

/// Deserialize a `Vec<T>` that may arrive as a JSON array OR a JSON string containing an array.
pub fn string_or_vec<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Array(arr)) => serde_json::from_value(Value::Array(arr))
            .map(Some)
            .map_err(serde::de::Error::custom),
        Some(Value::String(s)) => {
            let s = s.trim();
            if s.is_empty() {
                return Ok(None);
            }
            serde_json::from_str::<Vec<T>>(s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        Some(other) => Err(serde::de::Error::custom(format!(
            "expected array or string containing array, got {other}"
        ))),
    }
}

/// Deserialize a JSON value that may arrive as a native object/value OR a JSON string.
pub fn string_or_object<'de, D>(deserializer: D) -> Result<Option<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => {
            let s = s.trim();
            if s.is_empty() {
                return Ok(None);
            }
            serde_json::from_str::<Value>(s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        Some(other) => Ok(Some(other)),
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::{json, Value};

    #[derive(Debug, Deserialize)]
    struct VecArgs {
        #[serde(default, deserialize_with = "super::string_or_vec")]
        items: Option<Vec<String>>,
    }

    #[derive(Debug, Deserialize)]
    struct ObjectArgs {
        #[serde(default, deserialize_with = "super::string_or_object")]
        metadata: Option<Value>,
    }

    #[test]
    fn accepts_native_and_stringified_arrays() {
        let native: VecArgs = serde_json::from_value(json!({ "items": ["a", "b"] })).unwrap();
        assert_eq!(native.items.unwrap(), vec!["a", "b"]);

        let stringified: VecArgs =
            serde_json::from_value(json!({ "items": "[\"a\",\"b\"]" })).unwrap();
        assert_eq!(stringified.items.unwrap(), vec!["a", "b"]);
    }

    #[test]
    fn accepts_native_and_stringified_objects() {
        let native: ObjectArgs = serde_json::from_value(json!({ "metadata": { "a": 1 } })).unwrap();
        assert_eq!(native.metadata.unwrap(), json!({ "a": 1 }));

        let stringified: ObjectArgs =
            serde_json::from_value(json!({ "metadata": "{\"a\":1}" })).unwrap();
        assert_eq!(stringified.metadata.unwrap(), json!({ "a": 1 }));
    }
}
