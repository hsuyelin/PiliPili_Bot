use crate::infrastructure::network::api::Api;
use crate::infrastructure::network::response::ApiResponse;
use crate::infrastructure::network::task::HttpMethod;
use reqwest::{Client, Error};

pub struct Provider {
    client: Client,
}

impl Provider {

    pub fn new() -> Self {
        Provider {
            client: Client::new(),
        }
    }

    pub async fn send_request(api: &Api) -> Result<ApiResponse, Error> {
        let url = format!("{}{}", api.base_url, api.path);
        let request_builder = match api.method {
            HttpMethod::GET => Provider::client.get(&url),
            HttpMethod::POST => Provider::client.post(&url),
            HttpMethod::PUT => Provider::client.put(&url),
            HttpMethod::DELETE => Provider::client.delete(&url),
        };

        let request_builder = request_builder.headers(api.headers.clone());
        let response = request_builder.send().await?;
        let api_response = ApiResponse::from(response).await?;

        Ok(api_response)
    }
}
