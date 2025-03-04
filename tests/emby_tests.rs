#[cfg(test)]
mod tests {
    use pilipili_bot::infrastructure::api::EmbyAPI;
    use pilipili_bot::infrastructure::network::*;
    use tokio;
    use pilipili_bot::infrastructure::network::curl_logger_plugin::CurlLoggerPlugin;

    #[tokio::test]
    async fn test_emby_api_request_with_provider() {
        let api = EmbyAPI::GetUser {
            user_id: "<USER_ID>".to_string(),
            api_key: "<API_KEY>".to_string(),
        };

        let provider = Provider::new(vec![Box::new(CurlLoggerPlugin)]);

        match provider.send_request(&api).await {
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
