#[cfg(test)]
mod tests {
    use std::sync::RwLockReadGuard;
    use pilipili_bot::infrastructure::config::Config;
    use pilipili_bot::infrastructure::logger::builder::LoggerBuilder;
    use pilipili_bot::infrastructure::logger::{LogLevel, LogRotation, LogWriter};

    #[test]
    fn test_load_existing_config() {
        let _guard = LoggerBuilder::default()
            .with_level(LogLevel::Debug)
            .with_writer(LogWriter::File)
            .with_rolling(LogRotation::Daily)
            .init();
        
        let config: RwLockReadGuard<'static, Config> = Config::get();

        assert!(!config.emby.base_url.is_empty(), "Base URL should not be empty");
        assert!(!config.emby.api_key.is_empty(), "API key should not be empty");
    }
}