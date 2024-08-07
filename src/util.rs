use serde_json::Value;

pub fn build_json_object(pairs: &[(&str, &str)]) -> Value {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        map.insert(key.to_string(), Value::String(value.to_string()));
    }
    Value::Object(map)
}