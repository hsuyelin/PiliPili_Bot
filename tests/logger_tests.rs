// tests/logger_tests.rs

#[cfg(test)]
mod tests {

    use pilipili_bot::logger::logger::Logger;

    #[test]
    fn test_log_rotation() {
        Logger::rotate_logs();
        Logger::debug(None, "Log rotation test.");
    }

    #[test]
    fn test_debug_log() {
        Logger::debug(None, "This is a debug log.")
    }
}