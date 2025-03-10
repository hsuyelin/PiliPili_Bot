#[cfg(test)]
mod tests {

    use pilipili_bot::infrastructure::logger::builder::LoggerBuilder;
    use pilipili_bot::infrastructure::logger::{LogLevel, LogRotation};

    #[test]
    fn test_log() {
        let _guard = LoggerBuilder::default()
            .with_level(LogLevel::Debug)
            .with_rolling(LogRotation::Daily)
            .init();

        tracing::debug!("This is a debug log.");
        tracing::info!("This is a info log.");
        tracing::warn!("This is a warn log.");
        tracing::error!("This is a error log.");
    }
}