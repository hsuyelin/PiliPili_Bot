use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Task {
    RequestPlain,
    RequestJson(Value),
    RequestParameters(HashMap<String, String>),
}
