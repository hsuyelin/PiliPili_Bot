#[cfg(test)]
mod tests {
    
    use pilipili_bot::infrastructure::logger::{Logger, LogLevel};

    #[test]
    fn test_log() {
        Logger::set_log_level(LogLevel::Debug);
        Logger::debug("[DEBUG-TEST]".to_string(), "This is a debug log.");
        Logger::info("[INFO-TEST]".to_string(), "This is a info log.");
        Logger::warn("[WARN-TEST]".to_string(), "This is a warn log.");
        Logger::error("[ERROR-TEST]".to_string(), "This is a error log.");
    }
}