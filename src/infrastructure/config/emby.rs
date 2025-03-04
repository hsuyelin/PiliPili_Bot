use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct EmbyConfig {
    pub base_url: String,
    pub api_key: String,
}

impl Default for EmbyConfig {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:8096".to_string(),
            api_key: "".to_string(),
        }
    }
}