use reqwest::{Client, Method};
use once_cell::sync::Lazy;

use crate::infrastructure::network::{Task, TargetType, Plugin};

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36")
        .build()
        .expect("Failed to build HTTP client")
});

pub struct Provider {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Provider {

    pub fn new(plugins: Vec<Box<dyn Plugin>>) -> Self {
        Self { plugins }
    }

    pub async fn send_request<T: TargetType>(&self, target: &T) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}",
            target.base_url().trim_end_matches('/'),
            target.path().trim_start_matches('/')
        );

        let mut request = CLIENT.request(match target.method() {
            crate::infrastructure::network::HttpMethod::Get => Method::GET,
            crate::infrastructure::network::HttpMethod::Post => Method::POST,
            crate::infrastructure::network::HttpMethod::Put => Method::PUT,
            crate::infrastructure::network::HttpMethod::Delete => Method::DELETE,
        }, &url);

        if let Some(headers) = target.headers() {
            let mut header_map = reqwest::header::HeaderMap::new();
            for (key, value) in headers {
                header_map.insert(key, value.parse().unwrap());
            }
            request = request.headers(header_map);
        }

        match target.task() {
            Task::RequestPlain => {}
            Task::RequestJson(json_body) => {
                request = request.json(&json_body);
            }
            Task::RequestParameters(params) => {
                request = request.query(&params);
            }
        }

        for plugin in &self.plugins {
            if let Some(cloned_request) = request.try_clone() {
                if let Ok(built_request) = cloned_request.build() {
                    plugin.on_request(&built_request);
                }
            }
        }

        let response = request.send().await;
        match &response {
            Ok(res) => {
                for plugin in &self.plugins {
                    plugin.on_response(res);
                }
            }
            Err(err) => {
                for plugin in &self.plugins {
                    plugin.on_error(err);
                }
            }
        }

        response
    }
}
