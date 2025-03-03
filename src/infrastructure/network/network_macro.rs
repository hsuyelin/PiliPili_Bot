#[macro_export]
macro_rules! create_api {
    ($base_url:expr, $path:expr, $method:expr, $task:expr) => {{
        let mut api = Api::new(
            $base_url.to_string(),
            $path.to_string(),
            $method,
            $task,
        );
        api.set_header("Content-Type", "application/json");
        api.set_header(
            "User-Agent",
            r#"
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
AppleWebKit/537.36 (KHTML, like Gecko) \
Chrome/133.0.0.0 Safari/537.36
            "#
        );
        api
    }};
}
