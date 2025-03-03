use crate::infrastructure::network::task::{Task, HttpMethod};
use std::collections::HashMap;

pub struct Api {
    pub base_url: String,
    pub path: String,
    pub method: HttpMethod,
    pub task: Task,
    pub headers: HashMap<String, String>,
}

impl Api {

    pub fn new(base_url: String, path: String, method: HttpMethod, task: Task) -> Self {
        Api {
            base_url,
            path,
            method,
            task,
            headers: HashMap::new(),
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn send_request(&self) -> Result<String, String> {
        unimplemented!()
    }
}