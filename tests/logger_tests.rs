/*

#[cfg(test)]
mod tests {

    use std::thread;
    use std::time::{Duration, Instant};
    
    use pilipili_bot::infrastructure::logger::{Logger, LogLevel};

    #[test]
    fn test_debug_log() {
        Logger::set_log_level(LogLevel::Debug);
        Logger::debug("".to_string(), "This is a debug log.")
    }

    #[test]
    fn test_debug_log_with_interval() {
        Logger::set_log_level(LogLevel::Debug);
        let start_time = Instant::now();

        while start_time.elapsed() < Duration::new(10, 0) {
            Logger::debug("".to_string(), "This is a debug log.");
            thread::sleep(Duration::from_millis(25));
        }
    }
}
 */