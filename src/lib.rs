use serde_json::{Map, Value};

const TARGET_KEYS: [&str; 5] = ["email", "title", "name", "body", "content"];

/// Recursively walks through a JSON-like structure, updating the `type` field for
/// the specified keys to `"semantic_text"`.
///
/// The traversal supports arbitrarily nested objects and arrays represented via
/// `serde_json::Value`. Whenever it encounters one of the target keys whose value
/// is an object, the function ensures that the nested `type` property exists and
/// is set to `"semantic_text"`.
pub fn set_semantic_text_types(root: &mut Value) {
    walk_value(root);
}

fn walk_value(value: &mut Value) {
    match value {
        Value::Object(map) => walk_object(map),
        Value::Array(items) => {
            for item in items {
                walk_value(item);
            }
        }
        _ => {}
    }
}

fn walk_object(map: &mut Map<String, Value>) {
    let keys: Vec<String> = map.keys().cloned().collect();

    for key in keys {
        if let Some(child) = map.get_mut(&key) {
            if TARGET_KEYS.iter().any(|candidate| candidate == &key) {
                ensure_type_field(child);
            }
            walk_value(child);
        }
    }
}

fn ensure_type_field(value: &mut Value) {
    if let Value::Object(obj) = value {
        obj.insert(
            "type".to_string(),
            Value::String("semantic_text".to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn updates_nested_type_fields() {
        let mut value = json!({
            "title": { "type": "plain_text" },
            "content": { "other": "data" },
            "nested": {
                "array": [
                    {"name": {"type": "different"}},
                    {"email": {}},
                    "string",
                ],
                "body": {"type": "ignored"}
            }
        });

        set_semantic_text_types(&mut value);

        assert_eq!(
            value,
            json!({
                "title": { "type": "semantic_text" },
                "content": { "type": "semantic_text", "other": "data" },
                "nested": {
                    "array": [
                        {"name": {"type": "semantic_text"}},
                        {"email": {"type": "semantic_text"}},
                        "string",
                    ],
                    "body": {"type": "semantic_text"}
                }
            })
        );
    }
}
