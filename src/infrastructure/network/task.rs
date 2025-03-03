use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, Clone)]
pub enum Task {
    None,
    JSONBody(String),
    FormData(HashMap<String, String>),
    FileUpload(String),
}

impl Task {

    pub fn serialize(&self) -> Option<String> {
        match self {
            Task::JSONBody(body) => Some(body.clone()),
            Task::FormData(form_data) => {
                let serialized = serde_json::to_string(form_data).ok();
                serialized
            }
            _ => None,
        }
    }
}
