use crate::infrastructure::network::{HttpMethod, Task, TargetType};
use std::collections::HashMap;

pub enum EmbyAPI {
    GetUser { user_id: String, api_key: String },
}

impl TargetType for EmbyAPI {

    fn base_url(&self) -> &str {
        // <EMBY_DOMAIN> - Placeholder for now, will be read from the configuration file later
        "<EMBY_DOMAIN>"
    }

    fn path(&self) -> &str {
        match self {
            EmbyAPI::GetUser { user_id, .. } => {
                Box::leak(format!("emby/Users/{}", user_id).into_boxed_str())
            }
        }
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn task(&self) -> Task {
        match self {
            EmbyAPI::GetUser { api_key, .. } => {
                let mut params = HashMap::new();
                params.insert("api_key".to_string(), api_key.clone());
                Task::RequestParameters(params)
            }
        }
    }

    fn headers(&self) -> Option<Vec<(&'static str, &'static str)>> {
        Some(vec![
            ("accept", "application/json"),
            ("origin", "<EMBY_DOMAIN>"),
            ("referer", "<EMBY_DOMAIN>/"),
            ("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"),
        ])
    }
}
