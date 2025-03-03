#[cfg(test)]
mod tests {

    use pilipili_bot::infrastructure::network::*;
    use std::collections::HashMap;
    use reqwest::header::{HeaderMap, HeaderValue};

    #[tokio::test]
    async fn test_emby_api_request_with_provider() {
        let base_url = "https://emby.pilipiliultra.com";
        let path = "/emby/Users/56ed750c*********a2b3bd9c531e1a3";
        let method = HttpMethod::GET;
        let task = Task::None;

        match Provider::send_request(base_url, path, method, task).await {
            Ok(res) => {
                assert!(res.status().is_success(), "Request failed with status: {}", res.status());
                let body = res.text().await.unwrap();
                println!("Response body: {}", body);
                assert!(!body.is_empty(), "Response body is empty");
            }
            Err(e) => panic!("Request failed: {}", e),
        }
    }
}