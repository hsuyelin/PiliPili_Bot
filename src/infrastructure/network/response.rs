use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
}

impl ApiResponse {

    pub async fn from(response: Response) -> Result<ApiResponse, String> {
        let status_code = response.status().as_u16();
        let body = response.text().await.map_err(|e| e.to_string())?;
        Ok(ApiResponse { status_code, body })
    }

    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }
}
