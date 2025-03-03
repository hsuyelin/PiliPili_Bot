// tests/logger_tests.rs

#[cfg(test)]
mod tests {

    use pilipili_bot::logger::logger::Logger;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn test_log_rotation() {
        Logger::rotate_logs();
        Logger::debug(None, "Log rotation test.");
    }

    #[test]
    fn test_debug_log() {
        Logger::debug(None, "This is a debug log.")
    }

    #[test]
    fn test_debug_log_with_interval() {
        let start_time = Instant::now();

        while start_time.elapsed() < Duration::new(10, 0) {
            Logger::debug(None, "This is a debug log.");
            thread::sleep(Duration::from_millis(25));
        }
    }
}