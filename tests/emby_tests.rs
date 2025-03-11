#[cfg(test)]
mod tests {
    
    use tokio;

    use pilipili_bot::infrastructure::api::*;
    use pilipili_bot::infrastructure::logger::builder::LoggerBuilder;
    use pilipili_bot::infrastructure::logger::LogLevel;
    use pilipili_bot::infrastructure::network::*;

    #[tokio::test]
    async fn test_emby_api_request_with_provider() {
        LoggerBuilder::default()
            .with_level(LogLevel::Debug)
            .init();
        
        let api = EmbyAPI::GetUser {
            user_id: "56ed750c57e14553ba2b3bd9c531e1a3".to_string()
        };

        let provider = Provider::new(vec![Box::new(CurlPlugin)]);

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
